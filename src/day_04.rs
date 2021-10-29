use regex::Regex;

struct Duration {
    start: i32,
    end: i32,
    length: i32
}

struct Day {
    id: i32,
    month: i32,
    day: i32,
    durations: Vec<Duration>
}

impl Day {
    fn new() -> Day {
        Day {
            id: 0,
            month: 0,
            day: 0,
            durations: vec![]
        }
    }
}

struct Guard {
    id: i32,
    days: Vec<Day>
}

impl Guard {
    fn new(id: i32) -> Guard {
        Guard {
            id: id,
            days: vec![]
        }
    }
}

pub fn run(){
    let mut input: Vec<&str> = include_str!("../data/day_04.txt").lines().collect();
    input.sort();

    let reg_time = Regex::new(r"^\[1518-(\d+)-(\d+) (\d+):(\d+)\] (.*)$").unwrap();
    let reg_guard = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

    let mut cur_day: Day = Day::new();
    let mut start_sleep = 0;
    let mut days = vec![];

    for i in input {

        let cap = reg_time.captures(i).unwrap();
        if cap[5].starts_with("Guard") {

            if cur_day.id != 0 { days.push(cur_day); }

            cur_day = Day::new();
            cur_day.day = cap[2].parse::<i32>().unwrap();
            cur_day.month = cap[1].parse::<i32>().unwrap();
            if &cap[3] == "23" { cur_day.day += 1 };

            let guard_cap = reg_guard.captures(&cap[5]).unwrap();
            cur_day.id = guard_cap[1].parse::<i32>().unwrap();
        }

        if cap[5].starts_with("falls") {
            start_sleep = cap[4].parse::<i32>().unwrap();
        }

        if cap[5].starts_with("wakes") {
            cur_day.durations.push(Duration{
                start: start_sleep,
                end: cap[4].parse::<i32>().unwrap(),
                length: cap[4].parse::<i32>().unwrap() - start_sleep
            });
        }
    }

    days.push(cur_day);

    let mut guards: Vec<Guard> = vec![];
    let mut cur_guard: &mut Guard;

    for d in days {
        if let Some(guard) = guards.iter_mut().find(|c| (**c).id == d.id) {
            cur_guard = guard;
        } else {
            guards.push(Guard::new(d.id));
            cur_guard = guards.last_mut().unwrap();
        }

        cur_guard.days.push(d);
    }

    println!("Day 4");
    let a = part_a(&guards);
    println!("Part A result: {}", a);
    let b = part_b(&guards);
    println!("Part B result: {}", b);
}

fn part_a(guards: &Vec<Guard>) -> i32 {
    let mut sleep_time = 0;
    let mut id = 0;
    for g in guards {
        let mut cur_sleep_time = 0;
        for da in &g.days {

            for du in &da.durations {
                cur_sleep_time += du.length;
            }
        }

        if cur_sleep_time > sleep_time {
            id = g.id;
            sleep_time = cur_sleep_time;
        }
    }


    let mut minutes = [0; 60];

    for da in &guards.iter().find(|g| g.id == id).unwrap().days {
        for du in &da.durations {
            for i in du.start..du.end {
                minutes[i as usize] += 1;
            }
        }
    }


    let mut highest = 0;
    let mut most_sleep = 0;
    for m in 0..minutes.len() {
        if minutes[m] > highest {
            highest = minutes[m];
            most_sleep = m as i32;
        }
    }

    
    most_sleep * id
}

fn part_b(guards: &Vec<Guard>) -> i32 {
    let mut sleep_time = 0;
    let mut id = 0;
    let mut most_sleep = 0;

    for g in guards {
        let mut minutes = [0; 60];

        for da in &g.days {

            for du in &da.durations {
                for i in du.start..du.end {
                    minutes[i as usize] += 1;
                }
            }
        }

        for m in 0..minutes.len() {
            if minutes[m] > sleep_time {
                sleep_time = minutes[m];
                most_sleep = m as i32;
                id = g.id;
            }
        }
    }

    id * most_sleep
}