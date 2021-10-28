use regex::Regex;
use std::cmp;

struct Rectangle {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

pub fn run(){
    let input = include_str!("../data/day_03.txt");

    let mut rects: Vec<Rectangle> = vec![];
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for cap in re.captures_iter(input) {
        rects.push(
            Rectangle {
                id: cap[1].parse::<i32>().unwrap(), 
                x: cap[2].parse::<i32>().unwrap(), 
                y: cap[3].parse::<i32>().unwrap(), 
                width: cap[4].parse::<i32>().unwrap(), 
                height: cap[5].parse::<i32>().unwrap()
            }
        )
    }
    
    println!("Day 3");
    let a = part_a(&rects);
    println!("Part A result: {}", a);
    let b = part_b(&rects);
    println!("Part B result: {}", b);
}

fn part_a(rects: &Vec<Rectangle>) -> i32{

    // define and outline 
    let mut width = 0;
    let mut height = 0;
    for r in  rects {
        width = cmp::max(width, r.x + r.width);
        height = cmp::max(height, r.y + r.height)
    }

    let mut map: Vec<Vec<i16>> = vec![vec![0; width as usize]; height as usize];

    // fill map and count overlaps
    let mut counter = 0;
    for r in rects {
        for x in r.x..r.x+r.width {
            for y in r.y..r.y+r.height {
                if map[y as usize][x as usize] == 1 { counter += 1; }
                map[y as usize][x as usize] += 1
            }
        }
    }
    
    counter
}

fn rect_overlap(a: &Rectangle, b: &Rectangle) -> bool {
    !(a.x+a.width < b.x || a.x > b.x+b.width || a.y+a.height < b.y || a.y > b.y+b.height)
}

fn part_b(rects: &Vec<Rectangle>) -> i32{

    'outer: for a in rects {
        for b in rects {
            if a.id == b.id { continue; }
            if rect_overlap(&a, &b) { continue 'outer; }
        }
        return a.id;
    }
    
    println!("all overlap at least once");
    -1
}