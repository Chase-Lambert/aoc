pub fn day_01() {
    let input = include_str!("../resources/inputs/day_01.txt");

    println!("Day 1, Part 1: {:?}", part_1(&input));
    println!("Day 1, Part 2: {:?}", part_2(&input).unwrap());
}

fn part_1(directions: &str) -> i32 {
    let mut floor = 0;

    for d in directions.chars() {
        match d {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => unreachable!(),
        }
    }

    floor
}

fn part_2(directions: &str) -> Option<usize> {
    let mut position = 0;

    for (i, d) in directions.chars().enumerate() {
        match d {
            '(' => {
                position += 1;
            }
            ')' => {
                position -= 1;
            }
            _ => unreachable!(),
        }

        if position == -1 {
            return Some(i + 1);
        }
    }

    None
}
