extern crate regex;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let valid_entries = part_1(input);
    println!("{:#?}", valid_entries);
}

fn part_1(input: &str) -> usize {
    let mut passports = Vec::<Passport>::new();
    let mut last_line = String::new();
    for line in input.lines() {
        last_line.push(' ');
        last_line.push_str(line);

        // if we've encountered a new line we'll need to try
        // to construct a passport object with the information we have
        if line == "" {
            let mut pass = Passport::new();
            for entry in last_line.trim().split(" ") {
                let entry: Vec<&str> = entry.split(":").collect();
                let prop = entry[0];
                let val = entry[1];
                match prop {
                    "byr" => pass.byr = Some(val.into()),
                    "iyr" => pass.iyr = Some(val.into()),
                    "eyr" => pass.eyr = Some(val.into()),
                    "hgt" => pass.hgt = Some(val.into()),
                    "hcl" => pass.hcl = Some(val.into()),
                    "ecl" => pass.ecl = Some(val.into()),
                    "pid" => pass.pid = Some(val.into()),
                    "cid" => pass.cid = Some(val.into()),
                    other => panic!("unknown property: {:?}", other),
                }
            }
            if pass.is_valid() {
                passports.push(pass);
            }
            last_line.clear();
        }
    }

    passports.len()
}

#[derive(Debug)]
struct Passport {
    // Birth Year
    // four digits; at least 1920 and at most 2002.
    byr: Option<String>,
    // Issue Year
    // four digits; at least 2010 and at most 2020.
    iyr: Option<String>,
    // Expiration Year
    // four digits; at least 2020 and at most 2030.
    eyr: Option<String>,
    // Height
    // a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    hgt: Option<String>,
    // Hair Color
    // a # followed by exactly six characters 0 - 9 or a-f.
    hcl: Option<String>,
    // Eye Color
    // exactly one of: amb blu brn gry grn hzl oth.
    ecl: Option<String>,
    // Passport ID
    // a nine-digit number, including leading zeroes.
    pid: Option<String>,
    // Country ID
    // ignored, missing or not.
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.is_valid_byr()
            && self.is_valid_iyr()
            && self.is_valid_eyr()
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
            && self.is_valid_cid()
    }

    fn is_valid_byr(&self) -> bool {
        match &self.byr {
            Some(val) => { Passport::valid_year(val, 1920, 2002) }
            None => false,
        }
    }

    fn is_valid_iyr(&self) -> bool {
        match &self.iyr {
            Some(val) => { Passport::valid_year(val, 2010, 2020) }
            None => false,
        }
    }

    fn is_valid_eyr(&self) -> bool {
        match &self.eyr {
            Some(val) => { Passport::valid_year(val, 2020, 2030) }
            None => false,
        }
    }

    fn is_valid_hgt(&self) -> bool {
        match &self.hgt {
            Some(val) => {
                let re = Regex::new(r"(\d+)(cm|in)").unwrap();
                match re.captures(val) {
                    Some(caps) => {
                        let height = *&caps[1].parse::<i32>().unwrap_or(0);
                        let unit = &caps[2];
                        match unit {
                            "cm" => {
                                150 <= height && height <= 193
                            },
                            "in" => {
                                59 <= height && height <= 76
                            }
                            _ => false
                        }
                    },
                    None => false
                }
            },
            None => false
        }
    }

    fn is_valid_hcl(&self) -> bool {
        match &self.hcl {
            Some(val) => {
                let re = Regex::new(r"#[0-9,a-f]{6}").unwrap();
                re.is_match(val)
            },
            None => false
        }
    }

    fn is_valid_ecl(&self) -> bool {
        match &self.ecl {
            Some(val) => {
                match val.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false
                }
            }
            None => false
        }
    }

    fn is_valid_pid(&self) -> bool {
       match &self.pid {
           Some(val) => {
               let re = Regex::new(r"^\d{9}$").unwrap();
               re.is_match(val)
           },
           None => false
       }
    }

    fn is_valid_cid(&self) -> bool { true }

    fn valid_year(input: &str, min: i32, max: i32) -> bool {
        let year = input.parse::<i32>().unwrap_or(0);
        min <= year && year <= max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_byr() {
        let mut p = Passport::new();
        p.byr = Some("2002".into());
        assert_eq!(p.is_valid_byr(), true);
    }

    #[test]
    fn test_invalid_byr() {
        let mut p = Passport::new();
        p.byr = Some("2003".into());
        assert_eq!(p.is_valid_byr(), false);
    }

    #[test]
    fn test_valid_height_cm() {
        let mut p = Passport::new();
        p.hgt = Some("190cm".into());
        assert_eq!(p.is_valid_hgt(), true);
    }

    #[test]
    fn test_invalid_height_cm() {
        let mut p = Passport::new();
        p.hgt = Some("19cm".into());
        assert_eq!(p.is_valid_hgt(), false);
    }

    #[test]
    fn test_valid_height_in() {
        let mut p = Passport::new();
        p.hgt = Some("60in".into());
        assert_eq!(p.is_valid_hgt(), true);
    }

    #[test]
    fn test_invalid_height_in() {
        let mut p = Passport::new();
        p.hgt = Some("190in".into());
        assert_eq!(p.is_valid_hgt(), false);
    }

    #[test]
    fn test_invalid_height() {
        let mut p = Passport::new();
        p.hgt = Some("190".into());
        assert_eq!(p.is_valid_hgt(), false);
    }
}
