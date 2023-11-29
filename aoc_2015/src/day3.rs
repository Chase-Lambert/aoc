use std::collections::HashSet;

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_to_direction(c: char) -> Direction {
    match c {
        '^' => Direction::North,
        'v' => Direction::South,
        '>' => Direction::East,
        '<' => Direction::West,
        _ => unreachable!(),
    }
}

fn move_houses(current_house: &mut (i32, i32), direction: &Direction) {
    match direction {
        Direction::North => {
            current_house.0 += 1;
        }
        Direction::South => {
            current_house.0 -= 1;
        }
        Direction::East => {
            current_house.1 += 1;
        }
        Direction::West => {
            current_house.1 -= 1;
        }
    }
}

fn part_1(directions: &[Direction]) -> usize {
    let mut current_house = (0, 0);
    let mut houses_visited = HashSet::new();
    houses_visited.insert(current_house);

    for m in directions {
        move_houses(&mut current_house, m);
        houses_visited.insert(current_house);
    }

    houses_visited.len()
}

fn part_2(directions: &[Direction]) -> usize {
    let santa_directions: Vec<&Direction> = directions.to_owned().into_iter().step_by(2).collect();
    let robot_directions: Vec<&Direction> = directions
        .to_owned()
        .into_iter()
        .skip(1)
        .step_by(2)
        .collect();

    let mut current_house = (0, 0);
    let mut houses_visited = HashSet::new();
    houses_visited.insert(current_house);

    for m in santa_directions {
        move_houses(&mut current_house, m);
        houses_visited.insert(current_house);
    }

    current_house = (0, 0);

    for m in robot_directions {
        move_houses(&mut current_house, m);
        houses_visited.insert(current_house);
    }

    houses_visited.len()
}

pub fn day_03() {
    let input = include_str!("../resources/inputs/day_03.txt").trim();
    let directions: Vec<Direction> = input.chars().map(|c| move_to_direction(c)).collect();

    println!("Day 3, Part 1: {:?}", part_1(&directions)); // 2592
    println!("Day 3, Part 2: {:?}", part_2(&directions)); // 2360
}
