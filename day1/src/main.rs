
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn total_frequency(input_contents: &String) -> i32 {
    return input_contents
        .split('\n')
        .filter_map(|op| op.trim_left_matches('+').parse::<i32>().ok())
        .fold(0, |acc, op| acc + op);
}

fn first_multiple_freq(input_contents: &String) -> i32 {
    let arr : Vec<i32> = input_contents
        .split('\n')
        .filter_map(|op| op.trim_left_matches('+').parse::<i32>().ok())
        .collect();
    
    let mut total = 0;
    let mut totals = HashSet::new();

    loop {
        for op in &arr {
            total += op;
            
            if totals.contains(&total)   {
                return total;
            }

            totals.insert(total);
        }
    }
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
    
    let total = total_frequency(&contents);
    println!("final frequency: {}", total);

    let first_multiple = first_multiple_freq(&contents);
    println!("first repeated frequency: {}", first_multiple);
}
