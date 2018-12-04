
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Rect {
    location: Point,
    size: Point,
}

impl Rect {
    fn iter(&self) -> RectIter {
        return RectIter {
            location: self.location.clone(),
            size: self.size.clone(),
            current: Point { x: self.location.x - 1, y: self.location.y },
        };
    }

    fn left(&self) -> i32   {
        return self.location.x;
    }

    fn right(&self) -> i32  {
        return self.location.x + self.size.x;
    }

    fn top(&self) -> i32    {
        return self.location.y;
    }

    fn bottom(&self) -> i32 {
        return self.location.y + self.size.y;
    }

    fn overlap(&self, other: &Rect) -> bool {
        return  (other.left() <= self.right()) &&
                (self.left() <= (other.right())) && 
                (other.top() <= self.bottom()) &&
                (self.top() <= other.bottom()); 
    }
}

struct RectIter {
    location: Point,
    size: Point,
    current: Point,
}

impl Iterator for RectIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let min_x = self.location.x;
        let max_x = self.location.x + self.size.x;
        let max_y = self.location.y + self.size.y;

        if self.current.x + 1 >= max_x  {
            if self.current.y + 1 >= max_y  {
                None
            } else {
                self.current = Point { x: min_x, y: self.current.y + 1 };
                Some(self.current.clone())
            }
        } else {
            self.current = Point { x: self.current.x + 1, y: self.current.y };
            Some(self.current.clone())
        }
    }
}

struct ElfCut {
    id: u32,
    cut: Rect,
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

    let cuts : Vec<ElfCut> = contents
        .split('\n')
        .filter_map(|line| if line == "" {
            None
        } else {
            let sections : Vec<String> = line
                .split(' ')
                .map(|s| String::from(s))
                .collect();
            let coords : Vec<i32> = sections[2].trim_end_matches(':')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let size : Vec<i32> = sections[3]
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect();

            Some(
                ElfCut {
                    id: sections[0].trim_start_matches('#').parse().unwrap(),
                    cut: Rect {
                        location: Point { x: coords[0], y: coords[1] },
                        size: Point { x: size[0], y: size[1] },
                    },
                }
            )
        })
        .collect();

    let all_cuts : Vec<Point> = cuts
        .iter()
        .flat_map(|cut| cut.cut.iter())
        .collect();
    
    let max_width = all_cuts
        .iter()
        .map(|point| point.x)
        .max()
        .unwrap() + 1;
    
    let max_height = all_cuts
        .iter()
        .map(|point| point.y)
        .max()
        .unwrap() + 1;
    
    let mut arr = vec![0; (max_width * max_height) as usize];
    
    for point in all_cuts.iter() {
        arr[(point.x + (point.y * max_width)) as usize] += 1
    }
    
    let duplicates : u32 = arr
        .iter()
        .map(|count| if *count > 1 { 1 } else { 0 })
        .sum();

    println!("There are {} inches which are being cut more than once", duplicates);

    'outer: for x in 0..cuts.len()  {
        'inner: for y in 0..cuts.len()    {
            if x == y { continue 'inner; }
            if cuts[x].cut.overlap(&cuts[y].cut) {
                continue 'outer;
            }
        }

        println!("cut with id {} does not overlap any other cuts", cuts[x].id);
        break 'outer;
    }

    println!("took {:?}", start.elapsed());
}
