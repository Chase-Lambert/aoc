use std::collections::HashSet;

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_houses(current_house: &mut (i32, i32), direction: Direction) {
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

pub fn day_03() {
    let input = include_str!("../resources/inputs/day_03.txt");
    let input: Vec<char> = input.chars().collect();

    println!("Day 3, Part 1: {:?}", part_1(&input));
}

fn part_1(input: &[char]) -> usize {
    let mut current_house = (0, 0);
    let mut houses_visited = HashSet::new();
    houses_visited.insert(current_house);

    for m in input {
        let direction = match m {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!(),
        };

        move_houses(&mut current_house, direction);
        houses_visited.insert(current_house);
    }

    houses_visited.len()
}
