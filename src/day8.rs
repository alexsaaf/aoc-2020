use std::collections::HashSet;

#[derive(PartialEq, Clone)]
enum CommandType {
    NoOp,
    Accumulate,
    Jump
}

#[derive(Clone)]
pub struct Command {
    command_type: CommandType,
    value: i32
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str>  = line.split(" ").collect();
        let mut command_type: CommandType = CommandType::NoOp;
        match split_line[0] {
            "acc" => command_type = CommandType::Accumulate,
            "jmp" => command_type = CommandType::Jump,
            _ => ()
        }
        let sign = split_line[1].chars().next().unwrap();
        let val = split_line[1][1..].parse::<u32>().unwrap();

        result.push(Command {
            command_type: command_type,
            value: if sign == '+' { val as i32 } else { -1 * (val as i32) },
        });
    }

    return result;
}

#[aoc(day8, part1)]
pub fn part1(input: &[Command]) -> i32 {
    return run_code(input).0;
}

#[aoc(day8, part2)]
pub fn part2(input: &[Command]) -> i32 {
    let mut mutable_commands = input.to_vec();
    for command_index in 0..input.len()-1 {
        if mutable_commands[command_index].command_type == CommandType::Jump {
            mutable_commands[command_index].command_type = CommandType::NoOp;

            let res = run_code(&mutable_commands);
            if res.1 == input.len() {
                return res.0;
            }

            mutable_commands[command_index].command_type = CommandType::Jump;
        } else if mutable_commands[command_index].command_type == CommandType::NoOp{
            mutable_commands[command_index].command_type = CommandType::Jump;

            let res = run_code(&mutable_commands);
            if res.1 == input.len() {
                return res.0;
            }

            mutable_commands[command_index].command_type = CommandType::NoOp;
        }
    }

    return -1;
}

pub fn run_code(input: &[Command]) -> (i32, usize) {
    let mut visited_commands: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;

    let mut current_command_index: usize = 0;
    while !visited_commands.contains(&current_command_index) && current_command_index != input.len() {
        visited_commands.insert(current_command_index);
        
        let current_command = &input[current_command_index];
        match current_command.command_type {
            CommandType::Accumulate => {
                accumulator += current_command.value;
                current_command_index += 1;
            },
            CommandType::Jump => current_command_index = (current_command_index as i32 + current_command.value) as usize,
            _ => current_command_index += 1,
        }
    }

    return (accumulator, current_command_index);
}