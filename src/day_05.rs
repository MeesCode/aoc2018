use std::cmp;

pub fn run(){
    let input: String = String::from(include_str!("../data/day_05.txt").trim());
    
    println!("Day 5");
    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn react(input: &String) -> i32 {

    let mut chars = input.clone().into_bytes();

    'outer: loop{
        let mut change = false;
        for i in 0..chars.len()-1 {
            if i >= chars.len()-1 { continue 'outer; }
            if i16::abs(chars[i] as i16 - chars[i+1] as i16) == 32 {
                chars.remove(i);
                chars.remove(i);
                change = true;
            }
        }
        if !change { break; }
    }

    chars.len() as i32
}

fn part_a(input: &String) -> i32{
    react(input)
}

fn part_b(input: &String) -> i32{
    let mut smallest = input.len() as i32;

    for l in 'A'..'Z' {
        // println!("{}", l);
        let new_input = input.clone().replace(l, "").replace(l.to_lowercase().nth(0).unwrap(), "");
        smallest = cmp::min(smallest, react(&new_input));
    }
    
    smallest
}