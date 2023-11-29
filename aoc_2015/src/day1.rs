#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

#[aoc(day1, part1)]
fn part_1(directions: &[i32]) -> i32 {
    directions.iter().sum()
}

#[aoc(day1, part2)]
fn part_2(directions: &[i32]) -> Option<usize> {
    let mut position = 0;

    for (i, d) in directions.iter().enumerate() {
        position += d;
        if position == -1 {
            return Some(i + 1);
        }
    }

    None
}
