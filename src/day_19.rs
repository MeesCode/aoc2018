
#[derive(Clone, Copy)]
struct Command {
    fun: fn(&mut [usize; 6], &[usize; 3]),
    params: [usize; 3],
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_19.txt").trim());

    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut line_iter = lines.iter();
    let ip = line_iter.next().unwrap().split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

    let mut commands = Vec::new();
    for i in line_iter {
        let comp = i.split(" ").collect::<Vec<&str>>();
        commands.push(
            Command{
                fun: match comp[0] {
                    "addr" => addr, "addi" => addi,
                    "mulr" => mulr, "muli" => muli,
                    "banr" => banr, "bani" => bani,
                    "borr" => borr, "bori" => bori,
                    "setr" => setr, "seti" => seti,
                    "gtri" => gtri, "gtir" => gtir, "gtrr" => gtrr,
                    "eqri" => eqri, "eqir" => eqir, "eqrr" => eqrr,
                    _ => nope
                },
                params: [
                    comp[1].parse::<usize>().unwrap(),
                    comp[2].parse::<usize>().unwrap(),
                    comp[3].parse::<usize>().unwrap(),
                ]
            }
        )
    }

    // for i in &commands {
    //     println!("{} {} {}", i.params[0], i.params[1],i.params[2]);
    // }
    
    println!("Day 19"); 
    let a = part_a(ip, &commands);
    println!("Part A result: {}", a);
    let b = part_b(ip, &commands);
    println!("Part B result: https://www.hackmath.net/en/calculator/divisors?n={}", b);
}

fn part_a(ip: usize, commands: &Vec<Command>) -> usize {

    let mut regs: [usize; 6] = [0; 6];

    while regs[ip] < commands.len() {
        let index = regs[ip];
        let fun = commands[index].fun;
        fun(&mut regs, &commands[index].params);
        regs[ip] += 1;
        // println!("{:?}", regs);
    }

    regs[0]
}

// am lazy, don't want to implement algorithm
fn part_b(ip: usize, commands: &Vec<Command>) -> usize {

    let mut regs: [usize; 6] = [0; 6];
    regs[0] = 1;

    let mut s: u64 = 0;
    while regs[ip] < commands.len() {
        let index = regs[ip];
        let fun = commands[index].fun;
        fun(&mut regs, &commands[index].params);
        regs[ip] += 1;
        if s == 20 { break; }
        s += 1;
    }

    regs[2]
}

fn addr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] + regs[params[1]] }
fn addi(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] + params[1] }
fn mulr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] * regs[params[1]] }
fn muli(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] * params[1] }
fn banr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] & regs[params[1]] }
fn bani(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] & params[1] }
fn borr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] | regs[params[1]] }
fn bori(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] | params[1] }
fn setr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = regs[params[0]] }
fn seti(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = params[0] }
fn gtri(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if regs[params[0]] > params[1] { 1 } else { 0 } }
fn gtir(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if params[0] > regs[params[1]] { 1 } else { 0 } }
fn gtrr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if regs[params[0]] > regs[params[1]] { 1 } else { 0 } }
fn eqri(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if regs[params[0]] == params[1] { 1 } else { 0 } }
fn eqir(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if params[0] == regs[params[1]] { 1 } else { 0 } }
fn eqrr(regs:  &mut [usize; 6], params:  &[usize; 3]) { regs[params[2]] = if regs[params[0]] == regs[params[1]] { 1 } else { 0 } }
fn nope(_regs: &mut [usize; 6], _params: &[usize; 3]) { println!("no operation"); }
