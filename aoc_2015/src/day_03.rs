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
    println!("Day 3, Part 2: {:?}", part_2(&input));
}

fn part_1(directions: &[char]) -> usize {
    let mut current_house = (0, 0);
    let mut houses_visited = HashSet::new();
    houses_visited.insert(current_house);

    for m in directions {
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

fn part_2(directions: &[char]) -> usize {
    let santa_directions = directions
        .to_owned()
        .into_iter()
        .step_by(2)
        .collect::<Vec<char>>();
    let robot_directions = directions
        .to_owned()
        .into_iter()
        .skip(1)
        .step_by(2)
        .collect::<Vec<char>>();

    let mut current_house = (0, 0);
    let mut houses_visited = HashSet::new();
    houses_visited.insert(current_house);

    for m in santa_directions {
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

    current_house = (0, 0);

    for m in robot_directions {
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

mod tests {

    // println!("Day 3, Part 2: {:?}", part_2(&vec!['^', 'v',]));
    // println!("Day 3, Part 2: {:?}", part_2(&vec!['^', '>', 'v', '<']));
    // println!(
    //     "Day 3, Part 2: {:?}",
    //     part_2(&vec!['^', 'v', '^', 'v', '^', 'v', '^', 'v', '^', 'v'])
    // );
}
