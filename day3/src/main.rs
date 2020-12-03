fn main() {
    let input = include_str!("../input.txt");

    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let trees = part_1(&map);
    println!("part 1: {:?}", trees);

    let min_trees = part_2(&map);
    println!("part 1: {:?}", min_trees);
}

fn part_1(map: &[Vec<char>]) -> i32 {
    check_slope(3, 1, map)
}

fn part_2(map: &[Vec<char>]) -> i64 {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut acc: i64 = 1;
    for (dx, dy) in slopes.iter() {
        let trees = check_slope(*dx, *dy, map);
        acc = acc * trees as i64;
        // println!("slope ({}, {}) : {}", dx, dy, trees);
    }
    acc
}

fn check_slope(dx: i32, dy: i32, map: &[Vec<char>]) -> i32 {
    let mut pos: i32 = -dx; // to start on the first row @ 0
    let mut trees = 0;
    for step in map.iter().step_by(dy as usize) {
        pos = pos + dx;
        let width = step.len() as i32;
        let idx = (pos % width) as usize;
        if step[idx] == '#' {
            trees = trees + 1;
        }
    }
    trees
}
