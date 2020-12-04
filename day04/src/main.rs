use std::collections::HashMap;

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut passports = vec![];

    for line in instr.split("\n\n") {
        let line = line.trim();
        let passport: HashMap<&str, &str> = line
            .split(char::is_whitespace)
            .map(|field| {
                let mut iter = field.split(':');
                let key = iter.next().unwrap();
                let val = iter.next().unwrap();
                (key, val)
            })
            .collect();
        passports.push(passport);
    }

    let valid_cnt = passports
        .iter()
        // Input doesn't have invalid keys apparently:
        .filter(|&p| p.len() == 8 || (p.len() == 7 && !p.contains_key("cid")))
        .count();

    println!("Part 1: {}", valid_cnt);
}

fn p2_valid(p: &HashMap<&str, &str>) -> bool {
    {
        let p1 = p.len() == 8 || (p.len() == 7 && !p.contains_key("cid"));
        if !p1 {
            return false;
        }
    }

    {
        let byr: i32 = p["byr"].parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }
    }

    {
        let iyr: i32 = p["iyr"].parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
    }

    {
        let eyr: i32 = p["eyr"].parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }
    }

    {
        let hgt = p["hgt"];
        if hgt.contains("in") {
            let h: i32 = hgt.trim_end_matches("in").parse().unwrap();
            if h < 59 || h > 76 {
                return false;
            }
        } else if hgt.contains("cm") {
            let h: i32 = hgt.trim_end_matches("cm").parse().unwrap();
            if h < 150 || h > 193 {
                return false;
            }
        } else {
            return false;
        }
    }

    {
        let hcl = p["hcl"];
        if !hcl.starts_with('#') {
            return false;
        }
        let color = &hcl[1..];
        if color.len() != 6 {
            return false;
        }
        if !color.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    }

    {
        let ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let ecl = p["ecl"];
        if !ecls.contains(&ecl) {
            return false;
        }
    }

    {
        let pid = p["pid"];
        if pid.len() != 9 {
            return false;
        }

        if !pid.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
    }

    true
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut passports = vec![];

    for line in instr.split("\n\n") {
        let line = line.trim();
        let passport: HashMap<&str, &str> = line
            .split(char::is_whitespace)
            .map(|field| {
                let mut iter = field.split(':');
                let key = iter.next().unwrap();
                let val = iter.next().unwrap();
                (key, val)
            })
            .collect();
        passports.push(passport);
    }

    let valid_cnt = passports.iter().filter(|&p| p2_valid(p)).count();

    println!("Part 2: {}", valid_cnt);
}

fn main() {
    p1();
    p2();
}
