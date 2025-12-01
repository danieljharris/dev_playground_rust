// #![feature(explicit_tail_calls)]
// #![expect(incomplete_features)]
use std::fs::read_to_string;

// 

pub struct Map {
    source: u32,
    dest: u32,
    length: u32,
}
pub type Maps = Vec<Map>;
pub type MapOMaps = Vec<Maps>;

pub fn run() {
    let (seeds, meta_maps) = ingest();

    let mut lowest: u32 = u32::MAX;

    for seed in seeds {
        let current_value = hopper(&meta_maps, seed);
        if current_value < lowest {
            lowest = current_value;
        }
    }
    println!("Lowest is {:?}", lowest);
}

fn hopper(meta_maps: &Vec<Vec<Map>>, seed: u32) -> u32 {
    let mut result = seed;
    for maps in meta_maps {
        for map in maps {
            if result >= map.source && result <= (map.source + map.length) {
                result = (result - map.source) + (map.dest);
                break;
            }
        }
    }
    result
}

// fn lowest<T: Copy, S>(slice: &[T], init: S, f: impl Fn(S, T) -> S) -> S {
//     match slice {
//         [first, rest @ ..] => become lowest(rest, low(init, hopper(*first)), f),
//         [] => init,
//     }
// }

// fn low(a: u32, b: u32) -> u32 {
//     if a < b {
//         return a;
//     }
//     return b;
// }

pub fn ingest() -> (Vec<u32>, MapOMaps) {
    // let filename = "/workspaces/dev_playground_rust/src/seed_recursive/data/sample_data.txt".to_string();
    let filename = "/workspaces/dev_playground_rust/src/seed_recursive/data/full_data.txt".to_string();

    let mut seeds: Vec<u32> = Vec::new();
    let mut meta_maps: MapOMaps = Vec::new();
    let mut maps: Maps = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains("seeds:") {
            let segments: Vec<&str> = line["seeds: ".len()..].split(" ").collect();
            seeds = segments
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            // println!("Seeds: {:?}", seeds);
            continue;
        }
        if line.contains(":") {
            meta_maps.push(std::mem::take(&mut maps)); // move contents, reset maps to empty
            continue;
        }

        let numbers: Vec<u32> = line
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        maps.push(Map {
            source: numbers[1],
            dest: numbers[0],
            length: numbers[2],
        });
    }
    meta_maps.push(maps); // Push the final block

    return (seeds, meta_maps);
}
