use regex::Regex;
use std::cmp;

pub fn run(){
    let input = include_str!("../data/day_17.txt");
    let re = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)").unwrap();

    // find bounds
    let mut min_x = 500;
    let mut max_x = 500;
    let mut max_y = 0;
    let mut min_y = 999;
    for cap in re.captures_iter(input) {
        if cap[1] == *"x" {
            max_x = cmp::max(cap[2].parse::<usize>().unwrap(), max_x);
            min_x = cmp::min(cap[2].parse::<usize>().unwrap(), min_x);
            min_y = cmp::min(cap[4].parse::<usize>().unwrap(), min_y);
            max_y = cmp::max(cap[5].parse::<usize>().unwrap(), max_y);
        } else {
            max_x = cmp::max(cap[5].parse::<usize>().unwrap(), max_x);
            min_x = cmp::min(cap[4].parse::<usize>().unwrap(), min_x);
            max_y = cmp::max(cap[2].parse::<usize>().unwrap(), max_y);
            min_y = cmp::min(cap[2].parse::<usize>().unwrap(), min_y);
        }
    }

    let mut grid = vec![vec!['.'; max_x - min_x + 4]; max_y + 2];

    // println!("min x {} max x {}, min y {} max y {}", min_x, max_x, min_y, max_y);
    // println!("grid width {} grid height {}", grid[0].len(), grid.len());

    for cap in re.captures_iter(input) {
        let base = cap[2].parse::<usize>().unwrap();
        let from = cap[4].parse::<usize>().unwrap();
        let to = cap[5].parse::<usize>().unwrap();
        for i in from..to+1 {
            if cap[1] == *"x" {
                grid[i][(base as i32 - min_x as i32) as usize + 2] = '#';
            } else {
                grid[base][(i as i32 - min_x as i32) as usize + 2] = '#';
            }
        }
        grid[0][(500 - min_x) as usize + 1] = '+';
    }

    let mut water = -1;
    // _draw(&grid);

    for step in 0..1500 {

        if step % 100 == 0 {
            // println!("{}", step);
            let mut cur_water = 0;
            for y in min_y..grid.len()-1 {
                for x in 1..grid[0].len()-1 {
                    if grid[y][x] == '~' || grid[y][x] == '|'{
                        cur_water += 1;
                    }
                }
            }

            if cur_water == water {
                break;
            }

            water = cur_water;
        }

        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1{

                // drop water
                if (grid[y-1][x] == '+' || grid[y-1][x] == '|') && grid[y][x] == '.' {
                    grid[y][x] = '|';
                }

                // spread water
                if grid[y][x] == '|' && 
                (grid[y+1][x] == '~' || grid[y+1][x] == '#') {

                    let mut marked: Vec<(usize, usize)> = Vec::new();

                    // println!("spread");
                    
                    // check if enclosed and mark area
                    let mut enclosed = '~';
                    for t in x..grid[0].len() {
                        marked.push((t, y));
                        if grid[y+1][t] != '~' && grid[y+1][t] != '#' { 
                            enclosed = '|'; 
                            // println!("not enclosed right"); 
                            break; 
                        }
                        if grid[y][t+1] == '#' { break; }
                    }
                    for t in 0..x {
                        marked.push((x-t, y));
                        if grid[y+1][x-t] != '~' && grid[y+1][x-t] != '#' { 
                            enclosed = '|'; 
                            // println!("not enclosed left"); 
                            break; 
                        }
                        if grid[y][x-t-1] == '#' { break; }
                    }

                    // println!("enclosed: {}", enclosed);
                    // _draw(&grid);

                    // replace markers depending if enclosed
                    for i in &marked {
                        grid[i.1][i.0] = enclosed;
                    }

                }
                
            }
        }

        // println!();
        // _draw(&grid);
    }

    _draw(&grid);
    println!("{}", water);

    let mut settled = 0;

    for y in min_y..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == '~'{
                settled += 1;
            }
        }
    }

    println!("{}", settled);
    
    // println!("Day 1");
    // let a = part_a(&input);
    // println!("Part A result: {}", a);
    // let b = part_b(&input);
    // println!("Part B result: {}", b);
}

fn _draw(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}