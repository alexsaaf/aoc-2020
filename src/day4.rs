use regex::Regex;

#[derive(Default)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let mut current_passport: Passport = Default::default();
    let mut passports: Vec<Passport> = Vec::new();
    
    for line in input.lines() {
        if line == "" {
            if has_required_fields(&current_passport) {
                passports.push(current_passport);
            } 
            current_passport = Default::default();
        } else {
            let split_line = line.split(" ");
            for field in split_line {
                let mut split_field = field.split(':'); 
                let field_type: &str = split_field.next().unwrap();
                let value: &str = split_field.next().unwrap();   
                match field_type {
                    "byr" => current_passport.byr = value.to_string(),
                    "iyr" => current_passport.iyr = value.to_string(),
                    "eyr" => current_passport.eyr = value.to_string(),
                    "hgt" => current_passport.hgt = value.to_string(),
                    "hcl" => current_passport.hcl = value.to_string(),
                    "ecl" => current_passport.ecl = value.to_string(),
                    "pid" => current_passport.pid = value.to_string(),
                    "cid" => current_passport.cid = value.to_string(),
                    _ => ()
                }
            }
        }
    }
    if has_required_fields(&current_passport) {
        passports.push(current_passport);
    } 
    return passports;
}

pub fn has_required_fields(passport: &Passport) -> bool {
    return passport.byr != "" &&
           passport.iyr != "" &&
           passport.eyr != "" && 
           passport.hgt != "" && 
           passport.hcl != "" &&
           passport.ecl != "" && 
           passport.pid != "";
}


#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    return input.len();
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    return input.iter()
        .filter(|x| is_between(&x.byr, 1920, 2020))
        .filter(|x| is_between(&x.iyr, 2010, 2020))
        .filter(|x| is_between(&x.eyr, 2020, 2030))
        .filter(|x| {
            let height = &x.hgt;
            if height.ends_with("cm") {
                return is_between(&height[..(height.len() - 2)].to_string(), 150, 193); 
            } else if height.ends_with("in") {
                return is_between(&height[..(height.len() - 2)].to_string(), 59, 76); 
            } 
            return false;
        })
        .filter(|x| {
            let re = Regex::new(r"^#[a-z, 0-9]{6}$").unwrap();
            return re.is_match(&x.hcl);
        })
        .filter(|x| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&x.ecl[..]))
        .filter(|x| {
            let re = Regex::new(r"^[a-z, 0-9]{9}$").unwrap();
            return re.is_match(&x.pid);
        })
        .count();
}

fn is_between(year_string: &String, before: u32, after: u32) -> bool {
    let val = year_string.parse::<u32>().unwrap();
    return val >= before && val <= after;
}