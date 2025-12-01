pub fn run() {
    let values: Vec<(i32, &str)> = vec![(3, "Fizz"), (5, "Buzz"), (7, "Bang")];
    exe(values);
}

pub fn exe(values: Vec<(i32, &str)>) {
    for i in 1..=105 {
        println!("{}", fizz_stringafy(&values, i));
    }
}

fn fizz_stringafy(values: &[(i32, &str)], i: i32) -> String {
    let mut result: String = "".to_string();

    for (num, string) in values {
        if i % *num == 0 {
            result += string;
        }
    }

    if !result.is_empty() {
        return result;
    }
    
    return i.to_string();
}
