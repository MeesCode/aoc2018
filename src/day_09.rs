use regex::Regex;

pub fn run(){
    let input = include_str!("../data/day_09.txt").trim();

    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let cap = re.captures(input).unwrap();
    let players = cap[1].parse::<usize>().unwrap();
    let limit = cap[2].parse::<usize>().unwrap();

    
    println!("Day 1");
    let a = part_a(players, limit);
    println!("Part A result: {}", a);
    // let b = part_a(players, limit * 100);
    // println!("Part B result: {}", b);
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

fn part_a(players: usize, limit: usize) -> i32 {
    let mut current: usize = 0;
    let mut marbles = vec![0];
    let mut scores: Vec<i32> = vec![0; players];

    for value in 1..limit {

        if value % 23 == 0 {
            let remove_index = get_index(current as i32 - 7, marbles.len());
            let removed = marbles.remove(remove_index);
            scores[value % players] += removed + value as i32;
            current = remove_index;
        } else {
            let new_index = get_index(current as i32 + 2, marbles.len());
            marbles.insert(new_index, value as i32);
            current = new_index;
        }

        // println!("{}:\t{:?}", (player % players) + 1, marbles);

        if value % 10000 == 0 { println!("{}", value); }
    }

    let mut highest_score = 0;
    println!("{:?}", scores);
    for i in scores {
        if highest_score < i { highest_score = i; }
    }

    highest_score
}