use std::collections::HashSet;

#[derive(Default)]
pub struct Group {
    members: usize,
    selections: Vec<usize>
}

// This is a bit ugly. Didn't manage to convert char to ascii code in rust
const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(Group, HashSet<char>)> {
    let mut result: Vec<(Group, HashSet<char>)> = Vec::new();
    let mut current_set = HashSet::new();
    let mut current_group = Group {
        members: 0,
        selections: vec![0; 27]
    };
    for line in input.lines() {
        if line == "" {
            result.push((current_group, current_set));
            current_set = HashSet::new();
            current_group = Group {
                members: 0,
                selections: vec![0; 27]
            };
        } else {
            current_group.members += 1;
            for character in line.chars() {
                current_set.insert(character);
                current_group.selections[ALPHABET.iter().position(|x| *x == character).unwrap()] += 1;
            }
        }
    }
    result.push((current_group, current_set));
    return result;
}

#[aoc(day6, part1)]
pub fn part1(input: &[(Group, HashSet<char>)]) -> usize {
    return input.iter().map(|tuple| return tuple.1.len()).sum();
}

#[aoc(day6, part2)]
pub fn part2(input: &[(Group, HashSet<char>)]) -> usize {
    return input.iter().map(|tuple| {
        return tuple.0.selections.iter().filter(|selection| **selection == tuple.0.members).count();
    }).sum();
}