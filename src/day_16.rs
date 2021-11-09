use regex::Regex;

#[derive(Clone, Copy)]
struct Example {
    before: [usize; 4],
    command: [usize; 4],
    after: [usize; 4],
}

pub fn run(){
    let input = include_str!("../data/day_16.txt");

    let re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)]").unwrap();
    let mut examples: Vec<Example> = Vec::new();
    for cap in re.captures_iter(input) {
        examples.push(
            Example {
                before: [
                    cap[1].parse::<usize>().unwrap(), 
                    cap[2].parse::<usize>().unwrap(), 
                    cap[3].parse::<usize>().unwrap(), 
                    cap[4].parse::<usize>().unwrap()
                ],
                command: [
                    cap[5].parse::<usize>().unwrap(), 
                    cap[6].parse::<usize>().unwrap(), 
                    cap[7].parse::<usize>().unwrap(), 
                    cap[8].parse::<usize>().unwrap()
                ],
                after: [
                    cap[9].parse::<usize>().unwrap(), 
                    cap[10].parse::<usize>().unwrap(), 
                    cap[11].parse::<usize>().unwrap(), 
                    cap[12].parse::<usize>().unwrap()
                ]
            }
        )
    }

    let checks: Vec<fn(&[usize; 4], &[usize; 4]) -> usize> = vec![
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        setr, seti,
        gtri, gtir, gtrr,
        eqri, eqir, eqrr,
    ];

    let mut commands: Vec<[usize; 4]> = Vec::new();
    let rex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let program = input.split("\n\n\n").collect::<Vec<&str>>()[1];

    for cap in rex.captures_iter(program) {
        commands.push([
            cap[1].parse::<usize>().unwrap(), 
            cap[2].parse::<usize>().unwrap(), 
            cap[3].parse::<usize>().unwrap(), 
            cap[4].parse::<usize>().unwrap()
        ])
    }
    
    println!("Day 16");
    let a = part_a(&examples, &checks);
    println!("Part A result: {}", a);
    let b = part_b(&examples, &checks, &commands);
    println!("Part B result: {}", b);
}

fn part_a(examples: &Vec<Example>, checks: &Vec<fn(&[usize; 4], &[usize; 4]) -> usize>) -> i32 {

    let mut result = 0;

    for i in examples {
        let mut count = 0;
        for c in checks {
            let mut compare = i.after.clone();
            compare[i.command[3]] = c(&i.before, &i.command);
            if compare == i.after { count +=1 }
        }
        if count >= 3 { result+= 1; }
    }

    result
}

fn part_b(examples: &Vec<Example>, checks: &Vec<fn(&[usize; 4], &[usize; 4]) -> usize>, commands: &Vec<[usize; 4]>) -> usize {

    let mut possibllities: Vec<Vec<usize>> = vec![(0..16).collect(); checks.len()];

    for i in examples {
        let mut cur_pos: Vec<usize> = Vec::new();

        for (index, c) in checks.iter().enumerate() {
            let mut compare = i.after.clone();
            compare[i.command[3]] = c(&i.before, &i.command);

            if compare == i.after { 
                cur_pos.push(index);
            }
        }

        let mut new_p = Vec::new();
        for c in &cur_pos {
            if possibllities[i.command[0]].contains(c) {
                new_p.push(*c);
            }
        }
        possibllities[i.command[0]] = new_p;
    }


    let mut removed = Vec::new();
    for _ in 0..possibllities.len() {
        let mut one = 999;
        for p in &possibllities {
            if p.len() == 1 && !removed.contains(&p[0]){
                one = p[0];
            }
        }

        for p in 0..possibllities.len() {
            for i in 0..possibllities[p].len() {
                if possibllities[p][i] == one && possibllities[p].len() > 1{
                    possibllities[p].remove(i);
                    break;
                }
            }
        }

        removed.push(one);
    }

    let mut pointers = Vec::new();
    for i in &possibllities {
        pointers.push(checks[i[0]]);
    }

    let mut regs: [usize; 4] = [0,0,0,0];
    for i in commands {
        regs[i[3]] = pointers[i[0]](&regs, &i);
    }

    regs[0]
}

fn addr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] + regs[command[2]] }
fn addi(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] + command[2] }
fn mulr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] * regs[command[2]] }
fn muli(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] * command[2] }
fn banr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] & regs[command[2]] }
fn bani(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] & command[2] }
fn borr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] | regs[command[2]] }
fn bori(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] | command[2] }
fn setr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { regs[command[1]] }
fn seti(_regs: &[usize; 4], command: &[usize; 4]) -> usize { command[1] }
fn gtri(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if regs[command[1]] > command[2] { return 1; } else { return  0; } }
fn gtir(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if command[1] > regs[command[2]] { return 1; } else { return  0; } }
fn gtrr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if regs[command[1]] > regs[command[2]] { return 1; } else { return 0; } }
fn eqri(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if regs[command[1]] == command[2] { return 1; } else { return 0; } }
fn eqir(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if command[1] == regs[command[2]] { return 1; } else { return 0; } }
fn eqrr(regs:  &[usize; 4], command: &[usize; 4]) -> usize { if regs[command[1]] == regs[command[2]] { return 1; } else { return 0; } }

