use regex::Regex;
use std::collections::VecDeque;

pub fn run(){
    let input = include_str!("../data/day_09.txt").trim();

    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let cap = re.captures(input).unwrap();
    let players = cap[1].parse::<usize>().unwrap();
    let limit = cap[2].parse::<usize>().unwrap();

    
    println!("Day 9");
    let a = part_a(players, limit);
    println!("Part A result: {}", a);
    let b = part_a(players, limit * 100);
    println!("Part B result: {}", b);
}

fn get_index(index: i32, length: usize) -> usize{
    if index < 0 {
        return (length as i32 + index) as usize
    }

    if index > length as i32 {
        return index as usize - length
    }

    index as usize
}

fn part_a(players: usize, limit: usize) -> u64 {
    let mut marbles = VecDeque::new();
    let mut scores: Vec<u64> = vec![0; players];
    marbles.push_front(0);

    for value in 1..limit {

        if value % 23 == 0 {
            marbles.rotate_right(get_index(7, marbles.len()));
            let removed = marbles.remove(0).unwrap();
            scores[value % players] += removed + value as u64;
        } else {
            marbles.insert(get_index(2, marbles.len()), value as u64);
            marbles.rotate_left(2);
        }

        // println!("{}:\t{:?}", (value % players) + 1, marbles);
    }

    let mut highest_score = 0;
    for i in scores {
        if highest_score < i { highest_score = i; }
    }

    highest_score
}