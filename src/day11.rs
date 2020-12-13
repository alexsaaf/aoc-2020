use std::convert::TryInto;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    let lines = input.lines();
    for line in lines {
        let line_vec = line.chars().collect();
        res.push(line_vec);
    }

    return res;
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> u64 {
    let mut current_grid = input.to_vec(); 
    let mut next_grid = input.to_vec();
    
    let mut generational_change = true;
    while generational_change {
        generational_change = false;

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if current_grid[y][x] == 'L' && count_adjacent(&current_grid, (y, x)) == 0 {
                    next_grid[y][x] = '#';
                    generational_change = true;
                    continue;
                }
                if current_grid[y][x] == '#' && count_adjacent(&current_grid, (y, x)) >= 4 {
                    next_grid[y][x] = 'L';
                    generational_change = true;
                    continue
                }
            }
        }
        current_grid = next_grid.to_vec();
    }

    return current_grid.iter().fold(0, |acc, row| acc + row.iter().filter(|character| **character == '#').count() as u64);
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut current_grid = input.to_vec(); 
    let mut next_grid = input.to_vec();
    
    let mut generational_change = true;
    while generational_change {
        generational_change = false;

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if current_grid[y][x] == 'L' && count_visible(&current_grid, (y, x)) == 0 {
                    next_grid[y][x] = '#';
                    generational_change = true;
                    continue;
                }
                if current_grid[y][x] == '#' && count_visible(&current_grid, (y, x)) >= 5 {
                    next_grid[y][x] = 'L';
                    generational_change = true;
                    continue
                }
            }
        }
        current_grid = next_grid.to_vec();
    }

    return current_grid.iter().fold(0, |acc, row| acc + row.iter().filter(|character| **character == '#').count() as u64);
}

pub fn count_adjacent(grid: &Vec<Vec<char>>, position: (usize, usize)) -> i32 {
    let mut res = 0;
    let check_positions: [i32; 3] = [-1, 0, 1];

    for x_mod in &check_positions {
        for y_mod in &check_positions {
            if *x_mod == 0 && *y_mod == 0 {
                continue;
            }
            if position.1 as i32 + x_mod < 0 || position.1 as i32 + x_mod >= grid[0].len() as i32 {
                continue;
            }
            if position.0 as i32 + y_mod < 0 || position.0 as i32 + y_mod >= grid.len() as i32 {
                continue;
            }
            if grid[(position.0 as i32 + y_mod) as usize][(position.1 as i32 + x_mod) as usize] == '#' {
                res += 1;
            }
        }
    }
    return res;
}

pub fn count_visible(grid: &Vec<Vec<char>>, position: (usize, usize)) -> i32 {
    let mut res = 0;
    let check_positions: [i32; 3] = [-1, 0, 1];

    for x_mod in &check_positions {
        for y_mod in &check_positions {
            if *x_mod == 0 && *y_mod == 0 {
                continue;
            }

            let mut xPos: i32 = position.1.try_into().unwrap();
            let mut yPos: i32 = position.0.try_into().unwrap();
            let maxY = grid.len() - 1;
            let maxX = grid[0].len() - 1;

            xPos += x_mod;
            yPos += y_mod;

            while xPos as usize <= maxX && yPos as usize <= maxY {
                if grid[yPos as usize][xPos as usize] == '#' {
                    res += 1;
                    break;
                }
                if grid[yPos as usize][xPos as usize] == 'L' {
                    break;
                } 
                if *x_mod == -1 && xPos == 0 {
                    break;
                }
                if *y_mod == -1 && yPos == 0 {
                    break;
                }
                xPos += x_mod;
                yPos += y_mod;
            }
        }
    }
    return res;
}
