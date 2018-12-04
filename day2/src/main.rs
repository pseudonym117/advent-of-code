
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn letter_count(check_string: &str) -> HashMap<char, u32> {
    let mut letters : HashMap<char, u32> = HashMap::new();

    for letter in check_string.chars()  {
        if letters.contains_key(&letter) {
            let mut count = letters.get_mut(&letter).unwrap();
            *count += 1
        } else {
            letters.insert(letter, 1);
        }
    }

    return letters;
}

fn funky_hash(letters: &HashMap<char, u32>) -> (u32, u32)    {
    let mut duplicate = 0;
    let mut triplicate = 0;
    for (_, instances) in letters {
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
    
    let start = Instant::now();
    
    let ids : Vec<String> = contents
        .split('\n')
        .filter_map(|id| if id != "" {
            Some(String::from(id))
        } else {
            None
        })
        .collect();

    let hashes = ids
        .iter()
        .map(|id| funky_hash(&letter_count(id)))
        .fold((0, 0), |acc, check| (acc.0 + check.0, acc.1 + check.1));
    
    let checksum = hashes.0 * hashes.1;

    println!("Total checksum: {}", checksum);

    'search_loop: for x in 0..ids.len() {
        for y in (x + 1)..ids.len() {
            if ids[x].len() == ids[y].len()   {
                let filtered_string : String = ids[x].chars()
                    .zip(ids[y].chars())
                    .filter_map(|cs| if cs.0 == cs.1 {
                        Some(cs.0)
                    } else {
                        None
                    })
                    .collect();

                if ids[x].len() - filtered_string.len() == 1    {
                    println!("Common between IDs: {}", filtered_string);
                    break 'search_loop;
                }
            }
        }
    }

    println!("took {:?}", start.elapsed());
}
