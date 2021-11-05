use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down, 
    Left, 
    Right
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn corner(self, c: char) -> Direction{
        if c == '/' {
            match self {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
            }
        } else {
            match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
            }
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    turns: i32,
    crashed: bool
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            return self.x.cmp(&other.x)
        }
        self.y.cmp(&other.y)
    }
}

impl Cart {
    fn turn(&mut self) {
        self.turns += 1;
        let dir = self.direction.clone();
        match (self.turns-1) % 3 {
            0 => self.direction = dir.left(),
            2 => self.direction = dir.right(),
            _ => {}
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
        }
    }
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_13.txt"));
    let lines: Vec<&str> = input.lines().collect();

    let mut grid: Vec<Vec<char>> = vec![vec![' '; lines[0].len()]; lines.len()];

    for (i, l) in lines.iter().enumerate() {
        grid[i] = l.chars().collect();
    }

    let mut carts: Vec<Cart> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if ['>', 'v', '<', '^'].contains(&grid[y][x]) {
                let dir = match grid[y][x] {
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    _ => {panic!("lolwat")},
                };

                carts.push(
                    Cart {
                        direction: dir,
                        x: x,
                        y: y,
                        turns: 0,
                        crashed: false 
                    }
                );

                if grid[y][x] == '>' || grid[y][x] == '<' {
                    grid[y][x] = '-';
                }

                if grid[y][x] == '^' || grid[y][x] == 'v' {
                    grid[y][x] = '|';
                }
            }
        }
    }
    
    println!("Day 13");
    let ((x, y), (x2, y2)) = part_a(carts, &grid);
    println!("Part A result: {},{}", x, y);
    println!("Part B result: {},{}", x2, y2);
}

fn _display(carts: &Vec<Cart>, grid: &Vec<Vec<char>>) {
    let mut grid = grid.clone();
    
    for c in carts {
        if !['>', 'v', '<', '^'].contains(&grid[c.y][c.x]) && c.crashed {
            grid[c.y][c.x] = 'x';
            continue;
        }
        grid[c.y][c.x] = match c.direction {
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<'
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

fn part_a(mut carts: Vec<Cart>, grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)){

    // _display(&carts, &grid);
    let mut first_crash = (0,0);
    let mut last_cart = (0,0);
    let mut carts_left = carts.len();

    // _display(&carts, &grid);

    loop {

        carts.sort();

        if carts_left <= 1 {
            break;
        }

        for i in 0..carts.len() {

            if carts[i].crashed {
                continue;
            }

            carts[i].step();

            let dir = carts[i].direction.clone();
    
            match grid[carts[i].y][carts[i].x] {
                '/' | '\\' => carts[i].direction = dir.corner(grid[carts[i].y][carts[i].x]),
                '+' => carts[i].turn(),
                _ => {}
            }

            for j in 0..carts.len() {
                if i == j { continue; }
                if carts[i].x == carts[j].x && carts[i].y == carts[j].y && !carts[i].crashed && !carts[j].crashed {
                    carts[i].crashed = true;
                    carts[j].crashed = true;
                    carts_left -= 2;
                    if carts_left == carts.len() - 2 { first_crash = (carts[i].x, carts[i].y); }
                }
            }

        }

        // if s == 238 || s == 239 {
        //     println!();
        //     _display(&carts, &grid);
        // }
        
    }


    for i in carts {
        if !i.crashed {
            last_cart = (i.x, i.y);
        }
    }

    (first_crash, last_cart)

}