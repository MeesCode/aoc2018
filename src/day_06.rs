
use regex::Regex;
use std::cmp;

struct Point {
    id: i32,
    x: i32,
    y: i32,
    infinite: bool,
    area: i32
}

pub fn run(){
    let input = include_str!("../data/day_06.txt").trim();

    let mut points: Vec<Point> = vec![];
    let re = Regex::new(r"(\d+), (\d+)").unwrap();

    for (index, point) in re.captures_iter(input).enumerate() {
        points.push(
            Point{
                id: (index+1) as i32, 
                x: point[1].parse::<i32>().unwrap(),
                y: point[2].parse::<i32>().unwrap(),
                infinite: false,
                area: 0
            }
        )
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for i in &points {
        max_x = cmp::max(max_x, i.x);
        max_y = cmp::max(max_y, i.y);
    }

    let mut grid = vec![vec![0; max_x as usize]; max_y as usize];
    let mut distances = vec![vec![9999999; max_x as usize]; max_y as usize];

    for p in &points {
        for y in 0..max_y {
            for x in 0..max_x {
                let distance = i32::abs((p.x-1) - x) + i32::abs((p.y-1) - y);
                if distance < distances[y as usize][x as usize] {
                    grid[y as usize][x as usize] = p.id;
                    distances[y as usize][x as usize] = distance;
                } else if distance == distances[y as usize][x as usize] {
                    grid[y as usize][x as usize] = 0;
                }
            }
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {

            if grid[y as usize][x as usize] == 0 { continue; }

            if y == 0 || y == max_y-1 || x == 0 || x == max_x-1 {
                points[(grid[y as usize][x as usize]-1) as usize].infinite = true;
            }

            points[(grid[y as usize][x as usize]-1) as usize].area += 1;

        }
    }
    
    println!("Day 6");
    let a = part_a(&points);
    println!("Part A result: {}", a);
    let b = part_b(&points);
    println!("Part B result: {}", b);
}

fn part_a(points: &Vec<Point>) -> i32{

    let mut largest = 0;

    for i in points {
        if !i.infinite { largest = cmp::max(largest, i.area); }
    }

    largest
}

fn part_b(points: &Vec<Point>) -> i32{

    let mut max_x = 0;
    let mut max_y = 0;

    for i in points {
        max_x = cmp::max(max_x, i.x);
        max_y = cmp::max(max_y, i.y);
    }

    let mut count = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            let mut dist = 0;
            for p in points {
                dist += i32::abs((p.x) - x) + i32::abs((p.y) - y);
            }

            if dist < 10000 {
                count += 1;
            }
        }
    }

    count
}