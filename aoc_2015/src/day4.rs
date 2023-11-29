use md5;

fn find_md5_hash(key: &str, desired_leading_zeroes: usize) -> usize {
    for i in 0.. {
        let data = format!("{key}{i}");
        let digest = md5::compute(data);
        let first_n = format!("{digest:?}")
            .chars()
            .take(desired_leading_zeroes)
            .collect::<Vec<char>>();

        if first_n == vec!['0'; desired_leading_zeroes] {
            return i;
        }
    }

    0
}

fn part_1(key: &str) -> usize {
    find_md5_hash(key, 5)
}

fn part_2(key: &str) -> usize {
    find_md5_hash(key, 6)
}

pub fn day_04() {
    let key = include_str!("../resources/inputs/day_04.txt").trim();

    println!("Day 4, Part 1: {:?}", part_1(&key)); // 117946
    println!("Day 4, Part 2: {:?}", part_2(&key)); // 3938038
}
