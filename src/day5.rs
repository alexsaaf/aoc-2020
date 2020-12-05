#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for line in input.lines() {
        let (mut min_row, mut max_row) = (0, 127);
        // Figure out which row
        for character in line.chars().take(7) {
            if character == 'F' {
                max_row = take_front(&min_row, &max_row);
            } else {
                min_row = take_back(&min_row, &max_row);
            }
        }

        let (mut min_seat, mut max_seat) = (0, 7);
        // Figure out which seat
        for character in line.chars().skip(7).take(3) {
            if character == 'L' {
                max_seat = take_front(&min_seat, &max_seat);
            } else {
                min_seat = take_back(&min_seat, &max_seat);
            }
        }
        result.push(min_row * 8 + min_seat);
    }
    result.sort();
    return result;
}


#[aoc(day5, part1)]
pub fn part1(input: &[usize]) -> usize {
    return *input.iter().max().unwrap();
}

#[aoc(day5, part2)]
pub fn part2(input: &[usize]) -> usize {
    for i in 0..(input.len() - 1) {
        if input[i + 1] != input[i] + 1 {
            return input[i] + 1;
        }
    }
    return 0;
}

fn take_back(min: &usize, max: &usize) -> usize {
    let diff = max - min;
    let mid = (diff / 2) + min;
    let mid_index = mid as usize;
    return mid_index + 1;
}

fn take_front(min: &usize, max: &usize) -> usize {
    let diff = max - min;
    let mid = (diff / 2) + min;
    let mid_index = mid as usize;
    return mid_index - 1;
}