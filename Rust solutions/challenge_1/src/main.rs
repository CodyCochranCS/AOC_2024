use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (a, b) = get_lists_from_file("./input.txt");

    println!("{}", part1(a,b));
}

fn part1(a : Vec<i32>, b : Vec<i32>) -> i32 {
    let mut a = a.clone();
    let mut b = b.clone();
    a.sort();
    b.sort();

    a.iter().zip(b.iter()).map(|(&a,&b)|
        (a-b).abs()
    ).sum()
}

fn get_lists_from_file(filename : &str) -> (Vec<i32>, Vec<i32>) {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map_while(Result::ok)
            .map(|line| {
                let mut numbers = line
                    .split_ascii_whitespace()
                    .map(|x| 
                        x.parse::<i32>()
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