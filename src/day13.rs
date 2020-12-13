use num::Integer;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (i32, Vec<i32>) {
    let mut lines = input.lines();
    let earliest_time = lines.next().unwrap().parse::<i32>().unwrap();
    let busses = lines.next().unwrap().split(",").map(|time| {
        if time == "x" {
            return -1;
        } else {
            return time.parse::<i32>().unwrap();
        }
    }).collect();

    return (earliest_time, busses);
}


#[aoc(day13, part1)]
pub fn part1(input: &(i32, Vec<i32>) ) -> i32 {
    let mut shortest_wait_time = std::i32::MAX;
    let mut shortest_wait_time_bus_line = 0;
    for bus in 0..input.1.len() {
        if input.1[bus] == -1 {
            continue;
        }
        if find_departure_after(input.1[bus], input.0) - input.0 < shortest_wait_time {
            shortest_wait_time = find_departure_after(input.1[bus], input.0) - input.0;
            shortest_wait_time_bus_line = input.1[bus];
        }
    }
    return shortest_wait_time * shortest_wait_time_bus_line as i32;
}

pub fn find_departure_after(bus_start_time: i32, target_time: i32) -> i32 {
    return (target_time as f32 / bus_start_time as f32).ceil() as i32 * bus_start_time;
}

#[aoc(day13, part2)]
pub fn part2(input: &(i32, Vec<i32>) ) -> u64 {
    let mut timestamp: u64 = 0;
    let mut delta: u64 = 1;

    for bus_index in 0..input.1.len() {
        if input.1[bus_index] == -1 {
            continue;
        }

        while (timestamp + bus_index as u64) % input.1[bus_index] as u64 != 0 {
            timestamp += delta;
        }
        delta = delta.lcm(&(input.1[bus_index] as u64));
    }

    return timestamp;
}
