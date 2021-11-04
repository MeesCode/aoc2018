const GENERATIONS: usize = 50000000000;
const OFSSET: usize = 100;

#[derive(Clone)]
struct Note {
    pattern: String,
    next: char
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_12.txt").trim());
    let mut notes: Vec<Note> = Vec::new();
    let mut lines = input.lines();
    let initial = String::from(&lines.next().unwrap()[15..]);
    lines.next();

    for i in lines {
        if i.chars().collect::<Vec<char>>()[9] == '.' { continue; }
        notes.push(
            Note {
                pattern: String::from(i[0..5].chars().collect::<String>()),
                next: i.chars().collect::<Vec<char>>()[9]
            }
        );
    }

    let mut pots: Vec<char> = vec!['.'; OFSSET * 2 + initial.len()];
    for (i, c) in initial.chars().enumerate() {
        pots[OFSSET + i] = c;
    }

    // println!("initial state: {}\n", pots.iter().collect::<String>());
    // for i in &notes {
    //     println!("{} => {}", i.pattern, i.next);
    // }
    
    println!("Day 12");
    let a = part_a(&mut pots, &notes);
    println!("Part A result: {}", a);
    // let b = part_b(&input);
    // println!("Part B result: {}", b);
}

fn part_a(pots: &Vec<char>, notes: &Vec<Note>) -> i32 {

    let mut pots = pots.clone();
    let mut pots_mod;
    let mut result: i32 = 0;

    // println!("state: {}", pots.iter().collect::<String>());

    for g in 0..GENERATIONS {

        if g % 100 == 0 { println!("{}", g); }

        pots_mod = vec!['.'; pots.len()];
        result = 0;
        for n in notes {
            for x in 0..pots.len()-5 {
                let pot_pat = pots[x..x+5].iter().collect::<String>();
                if n.pattern == pot_pat {
                    pots_mod[x+2] = n.next;
                    result += (x + 2) as i32 - OFSSET as i32;
                }
            }
        }

        pots = pots_mod.clone();

        // println!("state: {}", pots.iter().collect::<String>());
    }

    result
}