use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let (mut a, mut b) = get_lists_from_file("./input.txt");
    
    let answer1 = part1(&mut a, &mut b);
    println!("Part 1: {}", answer1);

    let answer2 = part2(&a, &b);
    println!("Part 2: {}", answer2);
}

fn part1(a : &mut Vec<u32>, b : &mut Vec<u32>) -> u32 {
    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(&a,&b)| a.abs_diff(b))
        .sum()
}

fn part2(a : &Vec<u32>, b : &Vec<u32>) -> u32 {
    let b_counts: HashMap<&u32, usize> = b.iter().counts();

    a.iter()
        .map(|&x|
            x * *b_counts
                    .get(&x)
                    .unwrap_or(&0) as u32
        )
        .sum()
}

fn get_lists_from_file(filename : &str) -> (Vec<u32>, Vec<u32>) {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map_while(Result::ok)
            .map(|line| {
                let mut numbers = line
                    .split_ascii_whitespace()
                    .map(|x| 
                        x.parse::<u32>()
                        .unwrap()
                    );
                (numbers.next().unwrap(),numbers.next().unwrap())
                })
            .unzip()
    } else {
        (vec![], vec![])
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}