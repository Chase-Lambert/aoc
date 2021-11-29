fn contains_3_vowels(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let num_of_vowels = s
        .chars()
        .filter(|c| vowels.contains(c))
        .collect::<Vec<char>>()
        .len();

    num_of_vowels >= 3
}

fn contains_double_letter(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            return true;
        }
    }

    return false;
}

fn no_forbidden_strings(s: &str) -> bool {
    let forbidden_strings = ["ab", "cd", "pq", "xy"];

    for fs in forbidden_strings {
        if s.contains(fs) {
            return false;
        }
    }

    return true;
}

fn nice_string(s: &str) -> bool {
    contains_3_vowels(s) && contains_double_letter(s) && no_forbidden_strings(s)
}

fn part_1(data: &[&str]) -> usize {
    data.iter().filter(|s| nice_string(s)).count()
}

// fn part_2() {}

pub fn day_05() {
    let input = include_str!("../resources/inputs/day_05.txt").trim();
    let data: Vec<&str> = input.lines().collect();

    println!("Day 5, Part 1: {:?}", part_1(&data)); // 255
                                                    // println!("Day 5, Part 2: {:?}", part_2(&input));
}
