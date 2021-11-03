use regex::Regex;
use std::cmp;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl Point {
    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn step_reverse(&mut self) {
        self.x -= self.dx;
        self.y -= self.dy;
    }
}

pub fn run(){
    let input = include_str!("../data/day_10.txt").trim();
    let re = Regex::new(r"position=<([-\s]?\d+), ([-\s]?\d+)> velocity=<([-\s]?\d+), ([-\s]?\d+)>").unwrap();
    let mut points: Vec<Point> = Vec::new();

    for cap in re.captures_iter(input) {
        points.push(
            Point {
                x: cap[1].trim().parse::<i32>().unwrap(),
                y: cap[2].trim().parse::<i32>().unwrap(),
                dx: cap[3].trim().parse::<i32>().unwrap(),
                dy: cap[4].trim().parse::<i32>().unwrap()
            }
        )
    }
    
    println!("Day 10");
    println!("Part A result:");
    let b = part_a(&points);
    println!("Part B result: {}", b);
}

fn part_a(points: &Vec<Point>) -> i32 {
    let mut points = points.clone();
    let mut prev_area = area(&points);
    let mut counter = 0;

    loop {

        for i in &mut points {
            i.step();
        }

        let cur_area = area(&points);
        if cur_area > prev_area { break; }

        prev_area = cur_area;
        counter += 1;

    }

    for i in &mut points {
        i.step_reverse();
    }
    
    draw(&points);

    counter

}

fn area(points: &Vec<Point>) -> i64 {
    let mut min_x: i32 = 999999;
    let mut min_y: i32 = 999999;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for p in points {
        max_x = cmp::max(max_x, p.x);
        max_y = cmp::max(max_y, p.y);
        min_x = cmp::min(min_x, p.x);
        min_y = cmp::min(min_y, p.y);
    }

    i64::abs((max_x - min_x) as i64) * i64::abs((max_y - min_y) as i64) 
}

fn draw(points: &Vec<Point>){

    let mut min_x: i32 = 999999;
    let mut min_y: i32 = 999999;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for p in points {
        max_x = cmp::max(max_x, p.x);
        max_y = cmp::max(max_y, p.y);
        min_x = cmp::min(min_x, p.x);
        min_y = cmp::min(min_y, p.y);
    }

    let dist_x = i32::abs(max_x - min_x) + 1;
    let dist_y = i32::abs(max_y - min_y) + 1;

    let mut grid = vec![vec!['.'; dist_x as usize]; dist_y as usize];

    for p in points {
        grid[(p.y - min_y) as usize][(p.x - min_x) as usize] = '#';
    }

    for y in grid {
        println!("{}", y.into_iter().collect::<String>());
    }
}