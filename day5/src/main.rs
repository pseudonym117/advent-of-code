
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn react(polymer: &String) -> String {
    let mut output_chars : Vec<char> = polymer
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    
    loop {
        let mut reduced = Vec::new();

        let mut i = 1;
        while i < output_chars.len() {
            if output_chars[i - 1].is_ascii_uppercase() {
                if output_chars[i - 1].to_ascii_lowercase() == output_chars[i] {
                    i += 2;
                    continue;
                } else {
                    reduced.push(output_chars[i - 1]);
                    i += 1;
                }
            } else {
                if output_chars[i - 1].to_ascii_uppercase() == output_chars[i] {
                    i += 2;
                    continue;
                } else {
                    reduced.push(output_chars[i - 1]);
                    i += 1;
                }
            }
        }

        reduced.push(output_chars[output_chars.len() - 1]);

        if reduced.len() == output_chars.len() {
            break;
        }

        output_chars = reduced;
    }

    return output_chars.iter().collect();
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
    
    let start = Instant::now();

    let reduced = react(&contents);
    
    println!("Final element:\n{}", reduced);
    println!("{} total elements", reduced.len());
    
    let mut min_length = contents.len();
    let mut min_removed = '5';

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let removed = &reduced
            .chars()
            .filter(|x| x.to_ascii_lowercase() != c)
            .collect();
        
        let reacted = react(removed);
        let reacted_len = reacted.len();

        if reacted_len < min_length {
            min_length = reacted_len;
            min_removed = c;
        }
    }

    println!("remove \"{}\" produced the shortest string. Shortest length: {}", min_removed, min_length);

    println!("took {:?}", start.elapsed());
}
