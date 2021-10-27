use std::collections::HashSet;

pub fn run(){
    let input: String = String::from(include_str!("../data/day_02.txt"));
    
    println!("Day 2");
    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b.into_iter().collect::<String>());
}

fn part_a(input: &String) -> i32{
    let boxes = input.split_ascii_whitespace();

    let mut twos = 0;
    let mut threes = 0;

    for id in boxes {
        
        // init
        let mut letters = HashSet::new();
        let mut used = HashSet::new();

        // get all letters in string
        for letter in id.chars() {
            letters.insert(letter);
        }

        //count
        for check in letters {
            let mut counter = 0;

            for letter in id.chars() {
                if check == letter {
                    counter += 1;
                }
            }

            if used.contains(&counter){
                continue;
            }

            used.insert(counter);

            match counter {
                2 => twos += 1,
                3 => threes += 1,
                _ => ()
            }

        }

    }

    twos * threes
}

fn part_b(input: &String) -> Vec<char>{
    let boxes = input.split_ascii_whitespace();

    let boxes_array: Vec<&str> = boxes.collect();

    for check in 0..boxes_array.len() {
        for compare in 0..boxes_array.len() {
            
            // count different letters
            let mut counter = 0;
            let mut shared = vec![];

            let a: Vec<char> = boxes_array[check].chars().collect();
            let b: Vec<char> = boxes_array[compare].chars().collect();

            for i in 0..boxes_array[check].len(){
                if a[i] == b[i] {
                    shared.push(a[i]);
                } else {
                    counter += 1;
                }
            }

            if counter == 1 {
                return shared;
            }
        }
    }

    vec![]
}