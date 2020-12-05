use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let entries: Vec<&str> = input.lines().collect();

    let (min_id, max_id, seats) = part1(&entries);
    println!("part1: {:?}", max_id);

    let my_seat = part2(min_id, max_id, seats);
    println!("part2: {:?}", my_seat);
}

fn part1(entries: &[&str]) -> (usize, usize, Vec<usize>) {
    // NOTE: the trick here is to see that the seat numbers and ID's
    //       are just the binary representation of the BSP.
    let ids: Vec<usize> = entries.iter().map(|&line| {
        line.chars().rev().enumerate().fold(0, |acc: usize, (pos, chr)| {
            match chr {
                'B' | 'R' => { acc + (1 << pos) }
                _ => acc
            }
        })
    }).collect();
    (*ids.iter().min().unwrap(), *ids.iter().max().unwrap(), ids)
}

fn part2(min_id: usize, max_id: usize, seats: Vec<usize>) -> usize {
    let possible_seats: HashSet<usize> = (min_id..max_id+1).collect();
    let used_seats: HashSet<usize> = seats.iter().copied().collect();
    let mut my_seat: Vec<usize> = possible_seats.symmetric_difference(&used_seats).copied().collect();
    my_seat.sort();
    *my_seat.get(0).unwrap()
}