use std::fs::read_to_string;

pub struct Map {
    source: u32,
    dest: u32,
    length: u32,
}
pub type Maps = Vec<Map>;
pub type MapOMaps = Vec<Maps>;

pub fn run() {
    let (seeds, meta_maps) = ingest();

    for seed in seeds {
        // println!("Looking at seed: {seed}");
        let mut current_value = seed;

        for maps in &meta_maps {
            for map in maps {
                // println!("Looking At: Source: {:?}, Dest: {:?}, Length: {:?}", map.source, map.dest, map.length);

                if current_value >= map.source && current_value <= (map.source + map.length) {
                    let new_value = (current_value - map.source) + (map.dest);
                    // println!("  Moving from {current_value} to {new_value}");
                    // println!("      Looking At: Source: {:?}, Dest: {:?}, Length: {:?}", map.source, map.dest, map.length);
                    current_value = new_value;

                    break;
                }
            }
        }
        // println!("          seed {:?} maps to location {:?}", seed, current_value);
        println!("seed {:?} maps to location {:?}", seed, current_value);
    }
}

pub fn ingest() -> (Vec<u32>, MapOMaps) {
    let filename = "/workspaces/dev_playground_rust/src/seed/data/sample_data.txt".to_string();

    let mut seeds: Vec<u32> = Vec::new();
    let mut meta_maps: MapOMaps = Vec::new();
    let mut maps : Maps = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        if line.contains("seeds:") {
            let segments: Vec<&str> = line["seeds: ".len()..].split(" ").collect();
            seeds = segments.into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
            // println!("Seeds: {:?}", seeds);
            continue;
        }
        if line.contains(":") {
            meta_maps.push(std::mem::take(&mut maps)); // move contents, reset maps to empty
            continue;
        }

        let numbers: Vec<u32> = line.split(" ").into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
        maps.push(Map{source: numbers[1], dest: numbers[0], length: numbers[2]});
    }
    meta_maps.push(maps); // Push the final block

    return (seeds, meta_maps)
}