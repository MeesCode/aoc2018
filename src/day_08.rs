
struct Branch {
    index: usize,
    parent: usize,
    branch_count: i32,
    metadata_count: i32,
    branch_indexes: Vec<i32>,
    metadata: Vec<i32>
}

pub fn run(){
    let input: String = String::from(include_str!("../data/day_08.txt"));
    let items = input.split_ascii_whitespace();
    let mut numbers = vec![];
    for c in items {
        numbers.push(c.parse::<i32>().unwrap());
    }

    let mut branches: Vec<Branch> = vec![];

    branches.push(
        Branch {
            index: 0 as usize,
            parent: 0,
            metadata_count: numbers[1],
            metadata: vec![],
            branch_count: numbers[0],
            branch_indexes: vec![],
        }
    );

    let mut iter = numbers.iter().enumerate();
    iter.next();
    iter.next();

    let mut branch_started = 0;
    let mut branch_index: usize = 0;
    for (index, number) in iter {

        if index == branch_started + 1 {
            branches[branch_index].metadata_count = *number;
            continue;
        }

        if branches[branch_index].branch_count > branches[branch_index].branch_indexes.len() as i32 {
            let new_index = branches.len() as i32;
            branches[branch_index].branch_indexes.push(new_index);
            branch_started = index;
            let parent = branches[branch_index].index;

            branches.push(
                Branch {
                    index: branches.len(),
                    parent: parent,
                    metadata_count: 0,
                    metadata: vec![],
                    branch_count: *number,
                    branch_indexes: vec![],
                }
            );

            branch_index = branches.len() - 1;

            continue;
        }

        if branches[branch_index].metadata_count > branches[branch_index].metadata.len() as i32 {
            branches[branch_index].metadata.push(*number);

            if branches[branch_index].metadata_count == branches[branch_index].metadata.len() as i32 {
                branch_index = branches[branch_index].parent;
            }

            continue;
        }
        
    }

    println!("Day 8");
    let a = part_a(&branches);
    println!("Part A result: {}", a);
    let b = part_b(&branches);
    println!("Part B result: {}", b);
}

fn part_a(branches: &Vec<Branch>) -> i32 {

    let mut result = 0;
    for b in branches {
        for m in &b.metadata{
            result += m
        }
    }

    result
}

fn node_value(branches: &Vec<Branch>, index: usize) -> i32 {

    let mut result = 0;
    let node = &branches[index];

    if node.branch_count == 0 {
        for m in &node.metadata {
            result += m
        }
    } else {
        for m in &node.metadata {
            if m > &node.branch_count { continue; }

            result += node_value(&branches, node.branch_indexes[*m as usize - 1] as usize);
        }
    }

    result
}

fn part_b(branches: &Vec<Branch>) -> i32 {
    node_value(&branches, 0)
}