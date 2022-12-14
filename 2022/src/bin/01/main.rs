#![allow(dead_code)]

use std::fs::read_to_string;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod new_line {
    pub const SINGLE: &str = "\r\n";
    pub const DOUBLE: &str = "\r\n\r\n";
}

#[cfg(target_os = "linux")]
mod new_line {
    pub const SINGLE: &str = "\n";
    pub const DOUBLE: &str = "\n\n";
}

fn main() {
    let path = PathBuf::from("src/bin/01/input.txt");
    let input = read_to_string(path).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let answer: u32 = input
        .split(new_line::DOUBLE)
        .map(|s| s.split(new_line::SINGLE).filter_map(|s| s.parse::<u32>().ok()).sum())
        .max().unwrap();

    println!("Part 1: {:?}", answer);
}

fn part_two(input: &str) {
    let mut sums: Vec<u32> = input
        .split(new_line::DOUBLE)
        .map(|i| i.split(new_line::SINGLE).filter_map(|s| s.parse::<u32>().ok()).sum())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));

    let mut answer = 0;
    for i in 0..3 {
        answer += sums[i];
    };

    println!("Part 2: {:?}", answer);
}
