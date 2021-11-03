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
        return (length as i32 + (index % length as i32)) as usize;
    }
    index as usize % (length)
}

fn part_a(players: usize, limit: usize) -> i32 {
    let mut current: usize = 0;
    const ARRAY_SIZE: usize = 20;
    let mut marbles = vec![0; ARRAY_SIZE];
    let mut value = 1;
    let mut scores: Vec<i32> = vec![0; players];
    let mut array_length = 1;

    for player in 0..limit {

        if value % 23 == 0 {
            let remove_index = get_index(current as i32 - 7, array_length) % ARRAY_SIZE;
            let removed = marbles[remove_index];
            println!("removed: {}", removed);

            for i in remove_index..(array_length) {
                marbles[i % ARRAY_SIZE] = marbles[(i+1) % ARRAY_SIZE]
            }

            array_length -= 1;
            scores[player % players] += removed + value;
            current = remove_index;
        } else {
            let new_index = (get_index((current + 2) as i32, array_length)) % ARRAY_SIZE;
    
            if new_index == 0 {
                marbles[array_length % ARRAY_SIZE] = value;
                current = array_length;
            } else {

                for i in (new_index..array_length).rev() {
                    marbles[(i+1) % ARRAY_SIZE] = marbles[i % ARRAY_SIZE]
                }

                marbles[new_index] = value;
                current = new_index;
            }

            array_length += 1;
        }

        println!("{}:\t{:?}", (player % players) + 1, marbles);

        if value % 10000 == 0 { println!("{}", value); }
        value += 1;
    }

    let mut highest_score = 0;
    for i in scores {
        if highest_score < i { highest_score = i; }
    }

    highest_score
}