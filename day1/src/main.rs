use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result = part_1(input);
    println!("Part 1: {:?}", result);

    let result = part_2(input);
    println!("Part 2: {:?}", result);
}

fn part_1(input: &str) -> i32 {
    let values: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().expect("line is not an integer"))
        .collect();

    let (a, b) = find_pair_sum(&values, 2020).expect("no match?!");
    a * b
}

fn part_2(input: &str) -> i32 {
    let values: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().expect("line is not an integer"))
        .collect();

    let (a, b, c) = find_triple_sum(&values, 2020).expect("no match?!");
    a * b * c
}

fn find_pair_sum(values: &[i32], target: i32) -> Option<(i32, i32)> {
    // we'll keep track of all the values we've seen already which
    // haven't produced a match yet
    let mut sums = HashSet::<i32>::new();
    for val in values.iter() {
        // our guess for the matching number will then be the
        // difference between the number and it's target
        let guess = target - *val;
        // if we've seen the guessed value before then we know
        // that the guess and the current value sum to our guess
        if sums.contains(&guess) {
            return Some((guess, *val));
        } else {
            sums.insert(*val);
        }
    }
    None
}

fn find_triple_sum(values: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    // in the case in which we need three numbers we'll do an outer loop
    // on the numbers - 2 (to account for the ones we'll check in the inner loop)
    for idx in 0 .. values.len() - 2 {
        let mut sums = HashSet::<i32>::new();
        // the sub_sum is then basically the target for a similar method in
        // the find_double_sum(); that is to say we're looking for two numbers
        // in the sub-list which sum to the difference of the first number and
        // our actual target
        let sub_sum = target - values[idx];
        for idy in idx + 1 .. values.len() {
            let guess = sub_sum - values[idy];
            if sums.contains(&guess) {
                return Some((values[idx], values[idy], sub_sum - values[idy]));
            } else {
                sums.insert(values[idy]);
            }
        }
    }
    None
}
