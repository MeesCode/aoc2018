use std::collections::HashSet;
use std::cmp;

pub fn run(){
    let input_raw = include_str!("../data/day_20.txt").trim();
    let input = &input_raw[1..input_raw.len()].chars().collect::<Vec<char>>();

    let mut doors: HashSet<(i32, i32)> = HashSet::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    rec(0, (0, 0), input, &mut doors, &mut visited);

    // _draw(&doors, &visited);
    
    println!("Day 20");
    let (a, b) = part_ab(&doors);
    println!("Part A result: {}", a);
    println!("Part B result: {}", b);
}

fn part_ab(doors: &HashSet<(i32, i32)>) -> (i32, i32) {

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut branches: Vec<(i32, i32)> = Vec::new();
    branches.push((0,0));
    let mut depth = 0;
    let mut counter = 0;

    while !branches.is_empty() {

        let mut buffer: Vec<(i32, i32)> = Vec::new();

        for b in 0..branches.len() {
            let c = branches[b];
            visited.push(c);

            if depth >= 1000 {
                counter += 1;
            }

            if doors.contains(&(c.0, c.1-1)) && !visited.contains(&(c.0, c.1-2)) { buffer.push((c.0, c.1-2)) }
            if doors.contains(&(c.0, c.1+1)) && !visited.contains(&(c.0, c.1+2)) { buffer.push((c.0, c.1+2)) }
            if doors.contains(&(c.0-1, c.1)) && !visited.contains(&(c.0-2, c.1)) { buffer.push((c.0-2, c.1)) }
            if doors.contains(&(c.0+1, c.1)) && !visited.contains(&(c.0+2, c.1)) { buffer.push((c.0+2, c.1)) }
        }

        if buffer.is_empty() {
            break;
        }

        // println!("{:?}", buffer);

        branches = buffer;
        depth += 1;
    }

    (depth, counter)
}

fn _draw(doors: &HashSet<(i32, i32)>, visited: &HashSet<(i32, i32)>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for i in doors {
        min_x = cmp::min(min_x, i.0);
        min_y = cmp::min(min_y, i.1);
        max_x = cmp::max(max_x, i.0);
        max_y = cmp::max(max_y, i.1);
    }

    for y in min_y-1..max_y+2 {
        for x in min_x-1..max_x+2 {
            if x == 0 && y == 0 {
                print!("X");
            } else if doors.contains(&(x, y)){
                if y % 2 == 0 {
                    print!("|");
                } else {
                    print!("-");
                }
            } else {
                if visited.contains(&(x,y)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
        }
        println!();
    }
}

fn rec(index: usize, pos: (i32, i32), input: &Vec<char>, doors: &mut HashSet<(i32, i32)>, visited: &mut HashSet<(i32, i32)>) {
    let mut opens = 0;
    let mut indexes: Vec<usize> = vec![index];

    let mut cur_index = index;
    loop {
        if cur_index >= input.len()-1 { break; }
        match input[cur_index] {
            '(' => { opens += 1; },
            ')' => {
                if opens == 0 {
                    break;
                } else {
                    opens -= 1;
                }
            },
            '|' => {
                if opens == 0 {
                    indexes.push(cur_index + 1);
                }
            },
            _ => {}
        }
        cur_index += 1;
    }

    // println!("start {:?} index {}", pos, index);
    // println!("indexes {:?}", indexes);
    // println!();

    for i in indexes {
        let mut cur_index = i;

        let mut x = pos.0;
        let mut y = pos.1;

        loop {

            visited.insert((x, y));

            if cur_index >= input.len()-1 {
                break;
            }

            match input[cur_index] {
                'N' => {
                    doors.insert((x, y-1));
                    y -= 2;
                },
                'E' => {
                    doors.insert((x+1, y));
                    x += 2;
                },
                'S' => {
                    doors.insert((x, y+1));
                    y += 2;
                },
                'W' => {
                    doors.insert((x-1, y));
                    x -= 2;
                },
                '(' => {
                    rec(cur_index+1, (x, y), input, doors, visited);
                    break;
                },
                '|' => {
                    break;
                },
                _ => {}
            }

            cur_index += 1;
        }
    }
}