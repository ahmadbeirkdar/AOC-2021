use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Thanks @LesleyLai6
fn better_solve1(v: Vec<i32>) -> usize {
    return v.windows(2)
            .filter(|x| (x[1] - x[0]) > 0)
            .count();
}

fn better_solve2(v: Vec<i32>) -> usize {
    return v.windows(4)
            .filter(|x| (x[3] - x[0]) > 0)
            .count();
}

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

    println!("Ans1: {}", better_solve1(input.clone()));
    println!("Ans2: {}", better_solve2(input.clone()));
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}