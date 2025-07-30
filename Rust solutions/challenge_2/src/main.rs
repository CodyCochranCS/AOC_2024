use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let data = get_data_from_file("./input.txt");

    let answer1: u32 = data
        .iter()
        .map(|line| {
            let adjacent_test = |&a: &&i32, &b: &&i32| {
                let dif = a - b;
                1 <= dif && dif <= 3
            };
            (line.iter().is_sorted_by(adjacent_test) ||
            line.iter().rev().is_sorted_by(adjacent_test)) as u32
        })
        .sum();

    println!("Part 1: {}", answer1);
}

fn get_data_from_file(filename : &str) -> Vec<Vec<i32>> {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map_while(Result::ok)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| 
                        x.parse::<i32>()
                        .unwrap()
                    )
                    .collect()
                })
            .collect()
    } else {
        vec![]
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}