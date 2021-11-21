pub fn run(){
    let input: String = String::from(include_str!("../data/day_22.txt").trim());

    let lines = input.lines().collect::<Vec<&str>>();
    let depth = lines[0].split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
    let pos_line = lines[1].split(" ").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>();
    let target = (pos_line[0].parse::<usize>().unwrap(), pos_line[1].parse::<usize>().unwrap());

    println!("depth: {}\ntarget: {:?}", depth, target);

    

    // _draw(&grid);
    
    println!("Day 21");
    let a = part_a(depth, target);
    println!("Part A result: {}", a);
    // let b = part_b(depth, target);
    // println!("Part B result: {}", b);
}

fn part_a(depth: usize, target: (usize, usize)) -> i32 {
    let mut grid = vec![vec![0; target.0+1]; target.1+1];
    let mut risk = 0;

    for y in 0..target.1+1 {
        for x in 0..target.0+1 {
            let cur;
            if x == 0 && y == 0 { cur = depth % 20183; }
            else if x == target.0 && y == target.1 { cur = depth % 20183; }
            else if y == 0 { cur = (((16807 * x) + depth)) % 20183; }
            else if x == 0 { cur = (((48271 * y) + depth)) % 20183; }
            else { cur = ((grid[y][x-1] * grid[y-1][x]) + depth) % 20183; }

            grid[y][x] = cur;
            risk += cur as i32 % 3;
        }
    }

    risk
}

fn _draw(grid: &Vec<Vec<usize>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 && y == 0 { 
                print!("M");
            } else if x == grid[0].len()-1 && y == grid.len()-1 { 
                print!("T");
            } else {
                match grid[y][x] % 3 {
                    0 => { print!("."); }
                    1 => { print!("="); }
                    2 => { print!("|"); }
                    _ => {}
                }
            }
        }
        println!();
    }
}
