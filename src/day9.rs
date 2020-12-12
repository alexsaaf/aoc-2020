#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    return input.lines().map(|number| number.parse::<u64>().unwrap()).collect();
}


#[aoc(day9, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let mut current_index = 25;

    while can_sum_to(&input[(current_index - 25)..current_index], input[current_index]) {
        current_index += 1;
    }

    return input[current_index];
}


#[aoc(day9, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let mut current_index = 25; 
    while can_sum_to(&input[(current_index - 25)..current_index], input[current_index]) {
        current_index += 1;
    }
    let invalid_number = input[current_index];

    // Could skip the sums vector and only use the sum_vectors. Keeping for convenience ;) 
    let mut sums: Vec<u64> = Vec::new();
    let mut sum_vectors: Vec<Vec<u64>> = Vec::new();

    for number_index in 0..input.len() {
        for sum_index in 0..sums.len() {
            // No need to keep tracking too large sums
            if sums[sum_index] > invalid_number {
                continue;
            }

            sums[sum_index] += input[number_index];
            sum_vectors[sum_index].push(input[number_index]);

            if sums[sum_index] == invalid_number {
                return sum_vectors[sum_index].iter().max().unwrap() + sum_vectors[sum_index].iter().min().unwrap(); 
            }
        }

        sum_vectors.push(Vec::new());
        sum_vectors[number_index].push(input[number_index]);
        sums.push(input[number_index]);
    }
    return 0;
}


pub fn can_sum_to(available_numbers: &[u64], target: u64) -> bool {
    for i in 0..available_numbers.len() {
        for y in 0..available_numbers.len() {
            if available_numbers[i] != available_numbers[y] 
                && available_numbers[i] + available_numbers[y] == target {
                return true;
            }
        }
    }
    return false;
}

  