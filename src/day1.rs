#[aoc(day1, part1, Bytes)]
pub fn part1_bytes(input: &[u8]) -> i32 {
    input.iter().fold(0, |sum, c| match c {
        b'(' => sum + 1,
        b')' => sum - 1,
        _ => unreachable!(),
    })
}