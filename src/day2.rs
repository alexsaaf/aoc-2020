pub struct Rule {
    min: u32,
    max: u32,
    character: char,
    password: String
}

#[aoc_generator(day2)]
pub fn input_generator<'a>(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|l| {
            let line: Vec<&str> = l.trim().split(&[':', '-', ' '][..]).collect();
            let rule = Rule {
                min: line[0].parse::<u32>().unwrap(),
                max: line[1].parse::<u32>().unwrap(),
                character: line[2].chars().next().unwrap(),
                password: line[4].to_string(),
            };
            // println!("Line: {0}, {1}, {2}, {3}", rule.min, rule.max, rule.character, rule.password);
            return rule;
        }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Rule]) -> usize {
    let res = input.iter().filter(|r| {
        let count = r.password.matches(r.character).count() as u32;
        return count >= r.min && count <= r.max;
    });
    return res.count();
}

#[aoc(day2, part2)]
pub fn part2(input: &[Rule]) -> usize {
    let res = input.iter().filter(|r| {
        let first = r.password.chars().nth((r.min - 1) as usize).unwrap() == r.character;
        let second =  r.password.chars().nth((r.max - 1) as usize).unwrap() == r.character;

        return !(first && second) && (first | second)
    });
    return res.count();
}