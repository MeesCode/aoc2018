use std::collections::HashSet;

pub fn run(){
    let input: String = String::from(include_str!("../data/day_01.txt"));
    
    println!("Day 1");
    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn part_a(input: &String) -> i32{
    let parts = input.split_ascii_whitespace();

    let mut result = 0;

    for i in parts {
        result += i.parse::<i32>().expect("not a valid number")
    }

    result
}

fn part_b(input: &String) -> i32{
    let parts = input.split_ascii_whitespace();

    let mut result = 0;
    let mut freqs = HashSet::new();

    let mut numbers: Vec<i32> = vec![];

    for i in parts {
        numbers.push(i.parse::<i32>().expect("not a valid number"));
    }

    loop {
        for i in 0..numbers.len() {
            result += numbers[i];

            if freqs.contains(&result) {
                return result;
            }

            freqs.insert(result);
        }
    }
}