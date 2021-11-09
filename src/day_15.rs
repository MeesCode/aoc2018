use std::cmp::Ordering;

#[derive(Clone, Eq)]
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


impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl Unit {
    fn approach(&self, cave: &Vec<Vec<char>>, units: &Vec<Unit>) -> Option<(usize, usize)> {

        // find closest possible target location (breadth-first + read order)

        let mut us: Vec<(usize, usize)> = Vec::new();
        let mut others: Vec<(usize, usize)> = Vec::new();
        for i in units {
            if i.hp <= 0 { continue; }
            if i.team == self.team { us.push((i.x, i.y)); }
            else { others.push((i.x, i.y)); }
        }

        let mut targets: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut current: Vec<(usize, usize)> = Vec::new();
        let mut next: Vec<(usize, usize)> = Vec::new();

        current.push((self.x, self.y));

        while !current.is_empty() && targets.len() == 0 {
            // println!("1");

            while !current.is_empty() {
                // println!("2");
                let cur: (usize, usize) = current.pop().unwrap();
                visited.push((cur.0, cur.1));

                let mut steps: Vec<(usize, usize)> = Vec::new();
                if cur.0 > 0 { steps.push((cur.0-1, cur.1)); }
                if cur.0 < cave[0].len()-1 { steps.push((cur.0+1, cur.1)); }
                if cur.1 > 0 { steps.push((cur.0, cur.1-1)); }
                if cur.0 < cave.len()-1 { steps.push((cur.0, cur.1+1)); }

                for s in steps {
                    if others.contains(&s) && !targets.contains(&(cur.0, cur.1)) { targets.push((cur.0, cur.1)); continue; } 
                    if !visited.contains(&s) && cave[s.1][s.0] == '.' && !us.contains(&s) && !next.contains(&s) { next.push(s); }
                }
            }

            current = next.clone();
            next.clear();
        }

        if targets.len() == 0 {
            // println!("no moves possible");
            return None;
        }

        // println!("targets: {:?}", targets);

        let mut picked_target = (999 as usize, 999 as usize);
        for i in targets {
            if i.1 < picked_target.1 || (i.1 == picked_target.1 && i.0 < picked_target.0){
                picked_target = i;
            }
        }

        // println!("picked target: {:?}", picked_target);

        // pick shortsest path (reverse breadth-first + read order)

        current = vec![picked_target];
        targets = Vec::new();
        visited = Vec::new();
        let cur_cord = (self.x, self.y);

        while !current.is_empty() && targets.len() == 0 {
            // println!("3");

            while !current.is_empty() {
                // println!("4");
                let cur: (usize, usize) = current.pop().unwrap();
                visited.push((cur.0, cur.1));
                
                let mut steps: Vec<(usize, usize)> = Vec::new();
                if cur.0 > 0 { steps.push((cur.0-1, cur.1)); }
                if cur.0 < cave[0].len()-1 { steps.push((cur.0+1, cur.1)); }
                if cur.1 > 0 { steps.push((cur.0, cur.1-1)); }
                if cur.0 < cave.len()-1 { steps.push((cur.0, cur.1+1)); }

                for s in steps {
                    if cur_cord == s && !targets.contains(&s) { targets.push((cur.0, cur.1)); continue; }   

                    if !visited.contains(&s) && cave[s.1][s.0] == '.' && !others.contains(&s) && !us.contains(&s) && !next.contains(&s) { next.push(s); }
                }
            }

            current = next.clone();
            next.clear();
        }

        if targets.len() == 0 {
            // println!("no moves possible");
            return None;
        }

        // println!("moves: {:?}", targets);
        let mut picked_move = (999 as usize, 999 as usize);
        for i in targets {
            if i.1 < picked_move.1 || (i.1 == picked_move.1 && i.0 < picked_move.0){
                picked_move = i;
            }
        }
        // println!("picked move: {:?}", picked_move);

        // println!("{:?} to {:?}", (self.x, self.y), picked_move);

        Some(picked_move)
    }

    fn can_attack(&self, units: &Vec<Unit>) -> Option<usize> {
        let mut steps: Vec<(usize, usize)> = Vec::new();
        steps.push((self.x-1, self.y));
        steps.push((self.x+1, self.y));
        steps.push((self.x, self.y-1));
        steps.push((self.x, self.y+1));

        let mut lowest = 0;
        let mut lowest_hp = 999;

        for s in steps {
            for (i, u) in units.iter().enumerate() {
                if s == (u.x, u.y) && u.hp > 0 && u.hp < lowest_hp && u.team != self.team {
                    lowest = i;
                    lowest_hp = u.hp;
                }
            }
        }

        if lowest_hp == 999 {
            return None
        }

        // println!("{},{} attacks {},{}", self.x, self.y, units[lowest].x, units[lowest].y);

        Some(lowest)
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

    // _draw(&cave, &units);
    
    println!("Day 15");
    let a = part_a(&cave, &units);
    println!("Part A result: {}", a);
    // let b = part_b(&input);
    // println!("Part B result: {}", b);
}

fn _draw(cave: &Vec<Vec<char>>, units: &Vec<Unit>) {
    let mut cave = cave.clone();

    for i in units {
        if i.hp <= 0 { continue; }
        cave[i.y][i.x] = i.team;
    }

    for i in &cave {
        println!("{}", i.iter().collect::<String>());
    }
}

fn part_a(cave: &Vec<Vec<char>>, units: &Vec<Unit>) -> i32 {

    let mut units = units.clone();

    let mut counter = 0;
    // _draw(&cave, &units);
    loop {
        // println!("round {}", counter);

        for i in 0..units.len() {

            // println!("play unit {:?}", (units[i].x, units[i].y));

            if units[i].hp <= 0 { continue; }

            // _draw(&cave, &units);

            if let Some(index) = units[i].can_attack(&units) {
                units[index].hp -= 3;
                // println!("attack! new hp: {}", units[index].hp);
                continue;
            }

            if let Some(cord) = units[i].approach(&cave, &units) {
                units[i].x = cord.0;
                units[i].y = cord.1;

                if let Some(index) = units[i].can_attack(&units) {
                    units[index].hp -= 3;
                    // println!("attack! new hp: {}", units[index].hp);
                    continue;
                }
            }

            // for i in &units {
                // println!("{}", i.hp);
            // }
    
            let mut elf_hp = 0;
            let mut goblin_hp = 0;
            let mut total_hp = 0;
    
            for j in 0..units.len() {
                if units[j].team == 'E' && units[j].hp > 0 { elf_hp += units[j].hp; }
                if units[j].team == 'G' && units[j].hp > 0 { goblin_hp += units[j].hp; }
                if units[j].hp > 0 { total_hp += units[j].hp; }
            }
    
            if goblin_hp <= 0 || elf_hp <= 0 {
                // println!();
                // _draw(&cave, &units);

                // for i in &units {
                //     // println!("{}", i.hp);
                // }
                _draw(&cave, &units);
                return total_hp * counter;
            }

        }

        units.sort();

        counter += 1;

        // println!();
        // _draw(&cave, &units);

    }

}
