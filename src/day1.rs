#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<usize>) -> usize {
    let target = 2020;
    let mut found_numbers = vec![0; target];
    for item in input {
        if found_numbers[(target - item)] != 0 {
            return item * (target - item);
        }
        found_numbers[*item] = 1;
    }

    return 1;
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<usize>) -> usize {
    let target = 2020;
    for i in input.iter() {
        for y in input.iter() {
            for z in input.iter() {
                if (i + y + z) == target {
                    return i * y * z;
                }
            }
        }
    }
    return 1;
}