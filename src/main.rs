use websocket::{
    sync::Client,
    ClientBuilder,
    OwnedMessage,
    native_tls::TlsStream,
};
use std::net::TcpStream;
use serde_json::Value;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use priority_queue::PriorityQueue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Order {
    order_type: OrderType,
    price: i32,     // decimal shifted value stored for precision (4 decimal places)
    quantity: i32,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.order_type {
            OrderType::Buy => self.price.cmp(&other.price),  // high to low
            OrderType::Sell => other.price.cmp(&self.price), // low to high
        }
    }
}
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct OrderBook {
    buys: PriorityQueue<i32, i32>, 
    sells: PriorityQueue<i32, i32>,
    orders: HashMap<(OrderType, i32), Order>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            buys: PriorityQueue::new(),
            sells: PriorityQueue::new(),
            orders: HashMap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => {
                self.buys.push(order.price, order.quantity);
                self.orders.insert((OrderType::Buy, order.price), order);
            }
            }
            OrderType::Sell => {
                self.sells.push(order.price, order.quantity);
                self.orders.insert((OrderType::Sell, order.price), order);
            }
        }
    }

    fn upsert(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => {
                if order.quantity == 0 {
                    self.orders.remove(&(OrderType::Buy, order.price));
                }
                else if let Some(existing_order) = self.orders.get_mut(&(OrderType::Buy, order.price)) {
                    existing_order.quantity = order.quantity;
                }
                else {
                    self.add_order(order);
                }
            }
            OrderType::Sell => {
                if let Some(existing_order) = self.orders.get_mut(&(OrderType::Sell, order.price)) {
                    if order.quantity == 0 {
                        self.sells.remove(existing_order);
                        self.orders.remove(&(OrderType::Sell, order.price));
                    }
                    else {
                        existing_order.quantity = order.quantity;
                    }
                }
                else {
                    self.add_order(order);
                }
            }
        }
    }

    fn print_state(&self) {
        println!("BUYs: {:?}", self.buys);
        println!("SELLs: {:?}", self.sells);
    }
}


fn main() {
    let mut order_book = OrderBook::new();

    let server_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth";

    let mut client = ClientBuilder::new(server_url)
        .expect("Invalid WebSocket URL")
        .connect_secure(None)
        .expect("Failed to connect to server");
    println!("Connected to {}", server_url);

    handle_incoming_messages(&mut client, &mut order_book);
}

fn handle_incoming_messages(client: &mut Client<TlsStream<TcpStream>>, order_book: &mut OrderBook) {
    loop {
        let message = match client.recv_message() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        };

        match message {
            OwnedMessage::Text(txt) => handle_text_message(txt, order_book),
            OwnedMessage::Binary(bin) => println!("Binary: {:?}", bin),
            OwnedMessage::Ping(p) => {
                println!("Ping: {:?}", p);
                client
                    .send_message(&OwnedMessage::Pong(p))
                    .expect("Failed to send pong");
            }
            OwnedMessage::Close(frame) => {
                println!("Connection closed: {:?}", frame);
                break;
            }
            _ => {}
        }
    }
}

fn handle_text_message(txt: String, order_book: &mut OrderBook) {
    // println!("Message: {}\n\n", txt);

    if let Ok(json) = serde_json::from_str::<Value>(&txt) {
        if let Some(buys) = json.get("b") {
            // println!("Buys: {:?}", buys);
            for buy in buys.as_array().unwrap_or(&vec![]) {
                let price_str = buy[0].as_str().unwrap_or("0");
                let quantity_str = buy[1].as_str().unwrap_or("0");
                let price: i32 = (price_str.parse::<f64>().unwrap_or(0.0) * 10000.0) as i32;
                let quantity: i32 = quantity_str.parse::<i32>().unwrap_or(0);
                let order = Order {
                    order_type: OrderType::Buy,
                    price,
                    quantity,
                };
                // println!("Parsed Buy Order: {:?}", order);
                order_book.upsert(order);
            }
        }
        if let Some(sells) = json.get("a") {
            // println!("Sells: {:?}", sells);
            for sell in sells.as_array().unwrap_or(&vec![]) {
                let price_str = sell[0].as_str().unwrap_or("0");
                let quantity_str = sell[1].as_str().unwrap_or("0");
                let price: i32 = (price_str.parse::<f64>().unwrap_or(0.0) * 10000.0) as i32;
                let quantity: i32 = quantity_str.parse::<i32>().unwrap_or(0);
                let order = Order {
                    order_type: OrderType::Sell,
                    price,
                    quantity,
                };
                // println!("Parsed Sell Order: {:?}", order);
                order_book.upsert(order);
            }
        }
        order_book.print_state();
    } else {
        eprintln!("Failed to parse JSON message");
    }
}