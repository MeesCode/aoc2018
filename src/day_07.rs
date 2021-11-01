use regex::Regex;

struct Step {
    id: char,
    index: usize,
    requires: Vec<char>,
    time: i32,
}

pub fn run(){
    let input = include_str!("../data/day_07.txt").trim();

    let re = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
    let mut steps: Vec<Step> = vec![];
    let mut existing_steps: String = String::from("");

    for cap in re.captures_iter(input) {

        if !existing_steps.contains(&cap[1]) {
            let id = cap[1].chars().nth(0).unwrap();
            steps.push(
                Step{
                    id: id,
                    index: get_id(id),
                    requires: vec![],
                    time: completion_time(id),
                }
            );
            existing_steps = existing_steps + &cap[1];
        } 
        
        if !existing_steps.contains(&cap[2]) {
            let id = cap[2].chars().nth(0).unwrap();
            steps.push(
                Step{
                    id: id,
                    index: get_id(id),
                    requires: cap[1].chars().collect(),
                    time: completion_time(id),
                }
            );
            existing_steps = existing_steps + &cap[2];
        } else {
            if let Some(index) = steps.iter().position(|s| s.id == cap[2].chars().nth(0).unwrap()) {
                steps[index].requires.push(cap[1].chars().nth(0).unwrap());
            }
        }

    }

    // for s in &steps {
    //     println!("id: {}, time: {}, requires: {:?}", s.id, s.time, s.requires);
    // }

    println!("Day 7");
    let a = part_a(&steps);
    println!("Part A result: {}", a);
    let b = part_b(&steps);
    println!("Part B result: {}", b);
}

fn completion_time(c: char) -> i32{
    const MIN_TIME: i32 = 60;
    (String::from(c).into_bytes()[0]) as i32 - 64 + MIN_TIME
}

fn get_id(c: char) -> usize{
    ((String::from(c).into_bytes()[0]) as i32 - 64) as usize
}

fn part_a(steps: &Vec<Step>) -> String {

    let mut available: Vec<char> = vec![];
    let mut result: Vec<char> = vec![];

    for _ in 0..steps.len() {

        'outer: for s in steps {
            if result.contains(&s.id) || available.contains(&s.id) { continue; }
            for r in &s.requires {
                if !result.contains(&r) {
                    continue 'outer;
                }
            }
            available.push(s.id);
        }
        // println!("available: {:?}, done: {:?}", available, result);

        available.sort();
        result.push(available.swap_remove(0));
    }
    // println!("available: {:?}, done: {:?}", available, result);

    result.into_iter().collect::<String>()
}

fn get_available(steps: &Vec<Step>, times: &[i32], busy: &[bool]) -> Vec<usize> {

    let mut available: Vec<usize> = vec![];
    'outer: for s in steps {
        if busy[s.index] || times[s.index] == 0 { continue; }
        for r in &s.requires {
            if times[get_id(*r)] != 0 { continue 'outer; }
        }
        available.push(s.index);
    }

    available
}

fn done(times: &[i32]) -> bool {
    for s in times {
        if *s != 0 { return false; }
    }

    true
}

fn part_b(steps: &Vec<Step>) -> i32 {
    const WORKER_AMOUNT: usize = 5;
    let mut workers = 0;
    let mut times = [0; 27];
    let mut busy = [false; 27];

    for s in steps {
        times[s.index] = s.time;
    }

    let mut time = 0;
    while !done(&times) {

        for w in 0..27 {
            if times[w] == 0 && busy[w] { 
                busy[w] = false;
                workers -= 1;
            }
        }

        let available = get_available(&steps, &times, &busy);

        if available.len() > 0 && workers < WORKER_AMOUNT {
            let mut index = 0;
            while workers < WORKER_AMOUNT && index < available.len() {
                workers += 1;
                busy[available[index]] = true;
                index += 1;
            }
        }

        // println!("{}: {:?}", time, times);

        for s in steps {
            if busy[s.index] && times[s.index] != 0 { times[s.index] -= 1; }
        }

        time += 1;

    }

    // println!("{}: {:?}", time, times);

    time
}