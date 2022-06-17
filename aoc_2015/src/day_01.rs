fn part_1(directions: &[i32]) -> i32 {
    directions.iter().sum()
}

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

pub fn day_01() {
    let input = include_str!("../resources/inputs/day_01.txt").trim();
    let directions: Vec<i32> = input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect();

    println!("Day 1, Part 1: {:?}", part_1(&directions)); // 74
    println!("Day 1, Part 2: {:?}", part_2(&directions).unwrap()); // 1795
}
