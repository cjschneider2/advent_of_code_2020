#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = include_str!("../input.txt");

    let passwords: Vec<Password> = input
        .lines()
        .map(|line| {
            let (min, max, chr, pwd) = scan_fmt!(line, "{d}-{d} {}: {}", i32, i32, char, String).unwrap();
            Password { min, max, chr, pwd }
        })
        .collect();

    let result = part_1(&passwords);
    println!("Part 1: {:?}", result);

    let result = part_2(&passwords);
    println!("Part 2: {:?}", result);
}

#[derive(Debug, Clone)]
struct Password {
    min: i32,
    max: i32,
    chr: char,
    pwd: String
}

fn part_1(passwords: &[Password]) -> i32 {
    let mut valid_passwords = 0;
    for pwd in passwords {
        if pwd.check_char_count() {
            valid_passwords = valid_passwords + 1;
        }
    }
    valid_passwords
}

fn part_2(passwords: &[Password]) -> i32 {
    let mut valid_passwords = 0;
    for pwd in passwords {
        if pwd.check_char_position() {
            valid_passwords = valid_passwords + 1;
        }
    }
    valid_passwords
}

impl Password {
    fn check_char_count(&self) -> bool {
        let char_count = self.pwd.chars().fold(0, |acc, chr| {
            if chr == self.chr { acc + 1 } else { acc }
        });
        char_count >= self.min && char_count <= self.max
    }

    fn check_char_position(&self) -> bool {
        let p1 = self.pwd.chars().nth((self.min - 1) as usize).unwrap() == self.chr;
        let p2 = self.pwd.chars().nth((self.max - 1) as usize).unwrap() == self.chr;
        match (p1, p2) {
            (true, false) => true,
            (false, true) => true,
            _ => false
        }
    }
}

