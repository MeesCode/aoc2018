
pub fn run(){
    let input: String = String::from(include_str!("../data/day_14.txt"));
    
    println!("Day 14");
    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn part_a(input: &String) -> String {

    let iterations = input.parse::<usize>().unwrap();
    let mut recipes = vec![3,7];
    let (mut elf1, mut elf2): (usize, usize) = (0, 1);

    while recipes.len() <= iterations + 10 {
        elf1 = (elf1 + (recipes[elf1] as usize + 1)) % recipes.len();
        elf2 = (elf2 + (recipes[elf2] as usize + 1)) % recipes.len();

        let res = recipes[elf1] + recipes[elf2];

        if res >= 10 { recipes.push(1); }
        recipes.push(res % 10);
    }

    let mut result: String = String::new();
    for i in recipes[(iterations)..((iterations+10))].iter() {
        result += &i.to_string();
    }

    result
}

fn part_b(input: &String) -> usize {

    let mut seek = vec![];

    for i in input.chars() {
        seek.push(i.to_digit(10).unwrap() as u8);
    }

    let mut recipes = vec![3, 7];
    let (mut elf1, mut elf2): (usize, usize) = (0, 1);

    loop {

        if recipes.len() > seek.len() {
            if recipes[(recipes.len() - seek.len())..] == seek[..] {
                return recipes.len() - input.len();
            }

            if recipes[(recipes.len() - seek.len() - 1)..recipes.len()-1] == seek[..] {
                return recipes.len() - input.len() - 1;
            }
        }

        elf1 = (elf1 + (recipes[elf1] as usize + 1)) % recipes.len();
        elf2 = (elf2 + (recipes[elf2] as usize + 1)) % recipes.len();

        let res: u16 = recipes[elf1] as u16 + recipes[elf2] as u16;

        if res >= 10 { recipes.push(1); }
        recipes.push((res % 10) as u8);

    }

}