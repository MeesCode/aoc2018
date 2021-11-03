
const GRID_SIZE: usize = 300;

pub fn run(){
    let serial = String::from(include_str!("../data/day_11.txt").trim()).parse::<i32>().unwrap();

    let mut integral = [[0; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if x == 0 && y == 0 {
                integral[y][x] = power_lever(serial, x, y);
            } else if y == 0 {
                integral[y][x] = integral[y][x-1] + power_lever(serial, x, y);
            } else if x == 0 {
                integral[y][x] = integral[y-1][x] + power_lever(serial, x, y);
            } else {
                integral[y][x] = power_lever(serial, x, y) + integral[y][x-1] + integral[y-1][x] - integral[y-1][x-1];
            }
        }
    }
        
    println!("Day 11");
    let (a, b) = part_a(&integral);
    println!("Part A result: {},{}", a, b);
    let (a, b, c) = part_b(&integral);
    println!("Part B result: {},{},{}", a,b,c);
}

fn power_lever(serial: i32, x: usize, y: usize) -> i32 {
    (((((((x + 1) + 10) * (y + 1)) as i32 + serial) * ((x + 1) + 10) as i32) / 100) % 10) -5 as i32
}

fn find_highest(grid: &[[i32; GRID_SIZE]; GRID_SIZE], size: usize) -> ((usize, usize), i32) {
    let mut res = (0, 0);
    let mut highest = 0;
    let size = size-1;
    for y in 0..GRID_SIZE-size {
        for x in 0..GRID_SIZE-size {
            
            let value;
            if x == 0 && y == 0 {
                value = grid[y+size][x+size];
            } else if y == 0 {
                value = grid[y+size][x+size] - grid[y+size][x-1];
            } else if x == 0 {
                value = grid[y+size][x+size] - grid[y-1][x+size];
            } else {
                value = grid[y+size][x+size] + grid[y-1][x-1] - grid[y-1][x+size] - grid[y+size][x-1];
            }

            if value > highest {
                highest = value;
                res = (x+1, y+1);
            }
            
        }
    }
    (res, highest)
}

fn part_a(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize) {
    find_highest(&grid, 3).0
}

fn part_b(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, usize) {
    let mut res = (0,0);
    let mut highest = 0;
    let mut size = 0;

    for s in 1..300 {
        let (cord, value) = find_highest(&grid, s);
        if value > highest {
            res = cord;
            highest = value;
            size = s;
        }
    }

    (res.0, res.1, size)
}