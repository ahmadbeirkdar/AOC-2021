use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Sub {
    horz: i32,
    depth: i32,
    aim: i32,
}
impl Sub {
    fn new() -> Sub {
        Sub {horz: 0, depth: 0, aim: 0}
    }
    fn horz_by_depth(&self) -> i32 {
        self.horz * self.depth
    }
}

fn parse_to_directions(vec: Vec<&'static str>) -> Vec<(&'static str, i32)> {
    vec.into_iter()
        .map(|x| x.split(' '))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap().parse::<i32>().unwrap()))
        .collect()
}

fn solve1(vec: Vec<&'static str>) -> i32 {
    let vec = parse_to_directions(vec);
    vec.into_iter()
        .fold(Sub::new(), 
            |sub, (command, distance)|
                match command {
                    "forward" => Sub{horz: sub.horz + distance, ..sub},
                    "up"      => Sub{depth: sub.depth - distance, ..sub},
                    "down"    => Sub{depth: sub.depth + distance, ..sub},
                    _         => panic!("Error"),
                }
        )
        .horz_by_depth()
}

fn solve2(vec: Vec<&'static str>) -> i32 {
    let vec = parse_to_directions(vec);
    vec.into_iter()
        .fold(Sub::new(), 
            |sub, (command, distance)|
                match command {
                    "forward" => Sub{horz: sub.horz + distance, depth: sub.depth + sub.aim * distance, ..sub},
                    "up"      => Sub{aim: sub.aim - distance, ..sub},
                    "down"    => Sub{aim: sub.aim + distance, ..sub},
                    _         => panic!("Error"),
                }
        )
        .horz_by_depth()
}


fn main() {
    let mut input: Vec<&'static str> = Vec::new();

    if let Ok(lines) = read_lines("2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(Box::leak(ip.into_boxed_str()));
            }
        }
    }

    println!("Ans1: {}", solve1(input.clone()));
    println!("Ans2: {}", solve2(input));
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}