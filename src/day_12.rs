use std::collections::HashSet;

struct Note {
    pattern: Vec<char>,
    next: char
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_12.txt").trim());
    let mut notes: Vec<Note> = Vec::new();
    let mut lines = input.lines();
    let initial = String::from(&lines.next().unwrap()[15..]);
    lines.next();

    for i in lines {
        // if i.chars().collect::<Vec<char>>()[9] == '.' { continue; }
        notes.push(
            Note {
                pattern: i[0..5].chars().collect(),
                next: i.chars().collect::<Vec<char>>()[9]
            }
        );
    }

    let mut pots: HashSet<i32> = HashSet::new();
    for (i, c) in initial.chars().enumerate() {
        if c == '#' {
            pots.insert(i as i32);
        }
    }
    
    println!("Day 12");
    let a = part_a(&pots, &notes, 20);
    println!("Part A result: {}", a);
    let b = part_a(&pots, &notes, 50000000000);
    println!("Part B result: {}", b);
}

fn part_a(pots: &HashSet<i32>, notes: &Vec<Note>, generations: usize) -> u64 {

    let mut pots = pots.clone();
    let mut pots_mod: HashSet<i32>;
    let mut result: i32 = 0;
    let mut prev_result = result;
    let mut prev_diff = result - prev_result;

    let mut max = 0;
    let mut min = 0;
    for i in &pots {
        if *i < min { min = *i; }
        if *i > max { max = *i; }
    }

    for g in 0..generations {

        if g % 10 == 0 && g > 0 { 
            if result - prev_result == prev_diff {
                return (generations - g) as u64 * prev_diff as u64 + result as u64;
            }
            prev_diff = result - prev_result;
        }

        pots_mod = HashSet::new();
        prev_result = result;
        result = 0;
        for n in notes {
            'position: for rx in 0..max-min+6 {

                let x = rx as i32 + min - 5 as i32;

                for i in 0..5 {
                    if (n.pattern[i] == '#' && !pots.contains(&(x+i as i32))) || 
                       (n.pattern[i] == '.' &&  pots.contains(&(x+i as i32))) {
                        continue 'position;
                    }
                }

                let pot_nr = x as i32 + 2;
                if n.next == '#' {
                    pots_mod.insert(pot_nr);
                    result += x + 2;
                    if pot_nr < min { min = pot_nr; }
                    if pot_nr > max { max = pot_nr; }
                } else {
                    pots_mod.remove(&pot_nr);
                }
            }
        }

        pots = pots_mod.clone();
    }

    result as u64
}