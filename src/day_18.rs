
pub fn run(){
    let input: String = String::from(include_str!("../data/day_18.txt"));

    let mut grid: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        grid.push(l.chars().collect::<Vec<char>>());
    }
    for i in 0..grid.len() {
        grid[i].insert(0, '.');
        grid[i].push('.');
    }
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);
    
    println!("Day 18");
    let a = part_a(&grid);
    println!("Part A result: {}", a);
    let b = part_b(&grid);
    println!("Part B result: {}", b);
}

fn part_a(grid: &Vec<Vec<char>>) -> i32 {

    let mut grid = grid.clone();
    let mut grid_mod = grid.clone();

    for _step in 0..10 {

        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1 {
                
                let mut trees = 0;
                let mut lumberyards = 0;

                for dy in y-1..y+2 {
                    for dx in x-1..x+2 {
                        if dx == x && dy == y { continue; } 
                        if grid[dy][dx] == '|' { trees += 1; }
                        if grid[dy][dx] == '#' { lumberyards += 1; }
                    }
                }

                if grid[y][x] == '.' && trees >= 3 { grid_mod[y][x] = '|'; }
                if grid[y][x] == '|' && lumberyards >= 3 { grid_mod[y][x] = '#'; }
                if grid[y][x] == '#' && (lumberyards < 1 || trees < 1) { grid_mod[y][x] = '.'; }

            }
        }

        grid = grid_mod.clone();
    }

    let mut trees = 0;
    let mut lumberyards = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == '|' { trees += 1; }
            if grid[y][x] == '#' { lumberyards += 1; }
        }
    }

    trees * lumberyards
}

fn part_b(grid: &Vec<Vec<char>>) -> i32 {

    let mut grid = grid.clone();
    let mut grid_mod = grid.clone();

    let mut amounts: Vec<(i32, i32)> = Vec::new();

    const ITERATIONS: i32 = 1000000000;

    for _step in 0..100000000 {

        if _step >= 1000 {
            let mut trees = 0;
            let mut lumberyards = 0;
            for y in 1..grid.len()-1 {
                for x in 1..grid[0].len()-1 {
                    if grid[y][x] == '|' { trees += 1; }
                    if grid[y][x] == '#' { lumberyards += 1; }
                }
            }

            if amounts.contains(&(trees, lumberyards)){
                let todo = ITERATIONS - _step;
                let index = todo as usize % amounts.len();
                return amounts[index].0 * amounts[index].1;
            }

            amounts.push((trees, lumberyards));

        }

        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1 {
                
                let mut trees = 0;
                let mut lumberyards = 0;

                for dy in y-1..y+2 {
                    for dx in x-1..x+2 {
                        if dx == x && dy == y { continue; } 
                        if grid[dy][dx] == '|' { trees += 1; }
                        if grid[dy][dx] == '#' { lumberyards += 1; }
                    }
                }

                if grid[y][x] == '.' && trees >= 3 { grid_mod[y][x] = '|'; }
                if grid[y][x] == '|' && lumberyards >= 3 { grid_mod[y][x] = '#'; }
                if grid[y][x] == '#' && (lumberyards < 1 || trees < 1) { grid_mod[y][x] = '.'; }

            }
        }

        grid = grid_mod.clone();

    }

    0
}

fn _draw(grid: &Vec<Vec<char>>) {
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}