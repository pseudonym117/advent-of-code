
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn funky_hash(check_string: &str) -> (u32, u32)    {
    let mut letters : HashMap<char, u32> = HashMap::new();

    for letter in check_string.chars()  {
        if letters.contains_key(&letter) {
            let mut count = letters.get_mut(&letter).unwrap();
            *count += 1
        } else {
            letters.insert(letter, 1);
        }
    }

    let mut duplicate = 0;
    let mut triplicate = 0;
    for (_, instances) in &letters {
        match instances {
            2 => duplicate = 1,
            3 => triplicate = 1,
            _ => (),
        }
    }

    return (duplicate, triplicate);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];

    println!("reading input from \"{}\"...", filename);

    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading file");
    
    let hashes = contents
        .split('\n')
        .map(|id| funky_hash(id))
        .fold((0, 0), |acc, check| (acc.0 + check.0, acc.1 + check.1));
    
    let checksum = hashes.0 * hashes.1;

    println!("Checksum: {}", checksum);
}
