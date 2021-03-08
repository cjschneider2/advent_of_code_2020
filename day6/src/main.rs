use std::collections::HashSet;

type Group = HashSet<char>;

fn main() {
    //let input = include_str!("../test_input.txt");
    let input = include_str!("../input.txt");

    let sum_counts = part_1(input);
    println!("part 1: {}", sum_counts);

    let sum_counts = part_2(input);
    println!("part 2: {}", sum_counts);
}

fn part_1(input: &str) -> usize {
    let mut groups = Vec::<Group>::new();
    let mut last_group = Group::new();
    for line in input.lines() {
        if line == "" {
            groups.push(last_group.clone());
            last_group.clear()
        } else {
            for chr in line.chars() {
                last_group.insert(chr);
            }
        }
    }
    let sum_counts: usize = groups.iter().map(|group| {
        group.len()
    }).sum();
    sum_counts
}

fn part_2(input: &str) -> usize {
    let mut groups = Vec::<Group>::new();
    let mut last_group = Group::new();
    let mut start_group = true;
    for line in input.lines() {
        if line == "" {
            groups.push(last_group.clone());
            last_group.clear();
            start_group = true;
        } else {
            if start_group {
                for chr in line.chars() {
                    last_group.insert(chr);
                }
                start_group = false;
            } else {
                let current_entry = line.chars().collect::<Group>();
                last_group.retain(|x| current_entry.contains(&x));
            }
        }
    }
    let sum_counts: usize = groups.iter().map(|group| {
        group.len()
    }).sum();
    sum_counts
}