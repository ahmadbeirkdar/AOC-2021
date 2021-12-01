use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn solve1(v: Vec<i32>) -> usize {
    return v.iter()
                .zip(v.iter().skip(1))
                .map(|(cur, next)| next - cur)
                .filter(|&x| x > 0)
                .count();
}

fn solve2(v: Vec<i32>) -> usize {
    return v.iter()
                .zip(v.iter().skip(3))
                .map(|(cur, next)| next - cur)
                .filter(|&x| x > 0)
                .count();
}

fn main() {
    let mut input = Vec::new();

    if let Ok(lines) = read_lines("1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    
    println!("Ans1: {}", solve1(input.clone()));
    println!("Ans2: {}", solve2(input.clone()));
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}