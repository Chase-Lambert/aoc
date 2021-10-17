pub fn day_01() {
    let input = include_str!("../resources/inputs/day_01.txt");

    println!("Day 1, Part 1: {:?}", part_1(&input));
    println!("Day 1, Part 2: {:?}", part_2(&input).unwrap());
}

fn part_1(input: &str) -> i32 {
    let mut result = 0;

    for c in input.chars() {
        match c {
            '(' => {
                result += 1;
            }
            ')' => {
                result -= 1;
            }
            _ => unreachable!(),
        }
    }

    result
}

fn part_2(input: &str) -> Option<usize> {
    let mut result = 0;

    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => {
                result += 1;
            }
            ')' => {
                result -= 1;
            }
            _ => unreachable!(),
        }

        if result == -1 {
            return Some(idx + 1);
        }
    }

    None
}
