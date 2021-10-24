use md5;

pub fn day_04() {
    let input = include_str!("../resources/inputs/day_04.txt").trim();

    println!("Day 4, Part 1: {:?}", part_1(&input));
    println!("Day 4, Part 2: {:?}", part_2(&input));
}

fn part_1(key: &str) -> usize {
    for i in 0.. {
        let data = format!("{}{}", key, i);
        let digest = md5::compute(data);
        let first_five = format!("{:?}", digest)
            .chars()
            .take(5)
            .collect::<Vec<char>>();

        if first_five == vec!['0', '0', '0', '0', '0'] {
            return i;
        }
    }

    0
}

fn part_2(key: &str) -> usize {
    for i in 0.. {
        let data = format!("{}{}", key, i);
        let digest = md5::compute(data);
        let first_six = format!("{:?}", digest)
            .chars()
            .take(6)
            .collect::<Vec<char>>();

        if first_six == vec!['0', '0', '0', '0', '0', '0'] {
            return i;
        }
    }

    0
}
