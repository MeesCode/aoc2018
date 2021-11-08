use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, PartialOrd)]
struct Unit {
    team: char,
    hp: i32,
    x: usize,
    y: usize
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            return self.x.cmp(&other.x)
        }
        self.y.cmp(&other.y)
    }
}

impl Unit {
    fn approach(&self, cave: &Vec<Vec<char>>, units: &Vec<Unit>) -> (usize, usize) {

        // find closest possible target location (breadth-first + read order)
        let target = breadth_first_target(cave: &Vec<Vec<char>>, units: &Vec<Unit>, (self.x, self.y));


        // pick shortsest path (reverse breadth-first + read order)


        (0, 0)
    }

    fn can_attack(&self, units: &Vec<Unit>) -> Option<usize> {
        for (i, u) in units.iter().enumerate() {
            if (u.y == self.y+1 || u.y == self.y-1) &&
               (u.x == self.x+1 || u.x == self.x-1) &&
               u.team != self.team {
                   return Some(i);
               }
        }

        None
    }
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_15.txt"));
    let lines = input.lines().collect::<Vec<&str>>();

    let mut cave = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut units: Vec<Unit> = Vec::new();


    for (y, line) in lines.iter().enumerate(){
        for (x, c) in line.chars().collect::<Vec<char>>().iter().enumerate() {

            if *c == 'E' || *c == 'G' {
                units.push(
                    Unit{
                        team: *c,
                        hp: 200,
                        x: x,
                        y: y
                    }
                );

                cave[y][x] = '.';
                continue;
            }

            cave[y][x] = *c;
        }
    }

    _draw(&cave, &units);
    
    println!("Day 15");
    let a = part_a(&cave, &units);
    println!("Part A result: {}", a);
    // let b = part_b(&input);
    // println!("Part B result: {}", b);
}

fn _draw(cave: &Vec<Vec<char>>, units: &Vec<Unit>) {
    let mut cave = cave.clone();

    for i in units {
        cave[i.y][i.x] = i.team;
    }

    for i in &cave {
        println!("{}", i.iter().collect::<String>());
    }
}

fn part_a(cave: &Vec<Vec<char>>, units: &Vec<Unit>) -> i32 {
    0
}
