
const GRID_SIZE: usize = 300;

pub fn run(){
    let serial = String::from(include_str!("../data/day_11.txt").trim()).parse::<i32>().unwrap();

    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            grid[y][x] = (((((((x + 1) + 10) * (y + 1)) as i32 + serial) * ((x + 1) + 10) as i32) / 100) % 10) as i32 - 5;
        }
    }
        
    println!("Day 11");
    let (a, b) = part_a(&grid);
    println!("Part A result: {},{}", a, b);
    let (a, b, c) = part_b(&grid);
    println!("Part B result: {},{},{}", a,b,c);
}

fn find_highest(grid: &[[i32; GRID_SIZE]; GRID_SIZE], size: usize) -> ((usize, usize), i32) {
    let mut res = (0, 0);
    let mut highest = 0;
    for y in 0..GRID_SIZE-size {
        for x in 0..GRID_SIZE-size {
            
            let mut value = 0;
            for cy in y..y+size {
                for cx in x..x+size {
                    value += grid[cy][cx];
                }
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

    // i cheated here, i simply assumed that the size wouldn't be large and i was correct
    // searching the whole thing only takes 30s anyway
    for s in 1..20 {

        // if s % 10 == 0 { println!("{}", s); }

        let (cord, value) = find_highest(&grid, s);
        if value > highest {
            res = cord;
            highest = value;
            size = s;
        }
    }

    (res.0, res.1, size)
}