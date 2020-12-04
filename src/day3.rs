#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let y_size = input.lines().count();
    let x_size = input.lines().next().unwrap().len();

    let mut grid = vec![vec!['.'; x_size]; y_size];

    for (y_idx, line) in input.lines().enumerate() {
        for (x_idx, character) in line.chars().enumerate() {
            grid[y_idx][x_idx] = character;
        }
    }

    return grid;
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> usize {
    return count_trees(input, 3, 1);
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> usize {
    return count_trees(input, 1, 1) * 
           count_trees(input, 3, 1) *
           count_trees(input, 5, 1) * 
           count_trees(input, 7, 1) * 
           count_trees(input, 1, 2);
}

pub fn count_trees(grid: &Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize {
    let (mut y_pos, mut x_pos, mut trees) = (0, 0, 0);
    let height = grid.len();
    let width = grid[0].len();

    while y_pos != height - 1 {
        y_pos += y_step;

        if x_pos + x_step >= width {
            x_pos = (x_pos + x_step) - width;
        } else {
            x_pos += x_step;
        }

        if grid[y_pos][x_pos] == '#' {
            trees += 1;
        }
    }

    return trees;
}
