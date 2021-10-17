#[derive(Debug, Clone, Copy)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(dimensions: &str) -> Self {
        let dimensions: Vec<u32> = dimensions.split("x").map(|s| s.parse().unwrap()).collect();

        let (length, width, height) = (dimensions[0], dimensions[1], dimensions[2]);

        Present {
            length,
            width,
            height,
        }
    }

    fn surface_area(&self) -> u32 {
        let (length, width, height) = (self.length, self.width, self.height);

        (2 * length * width) + (2 * width * height) + (2 * height * length)
    }

    fn area_of_smallest_side(&self) -> u32 {
        let (length, width, height) = (self.length, self.width, self.height);
        let mut side_lengths: Vec<u32> = vec![length, width, height];
        side_lengths.sort();

        side_lengths[0] * side_lengths[1]
    }

    fn paper_needed(&self) -> u32 {
        self.surface_area() + self.area_of_smallest_side()
    }

    fn smallest_perimeter(&self) -> u32 {
        let (length, width, height) = (self.length, self.width, self.height);
        let mut side_lengths: Vec<u32> = vec![length, width, height];
        side_lengths.sort();

        side_lengths[0] * 2 + side_lengths[1] * 2
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn ribbon_needed(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

pub fn day_02() {
    let input = include_str!("../resources/inputs/day_02.txt");
    let input: Vec<&str> = input.lines().collect();

    println!("Day 2, Part 1: {:?}", part_1(&input));
    println!("Day 2, Part 1: {:?}", part_2(&input));
}

fn part_1(presents: &[&str]) -> u32 {
    let mut result = 0;
    for p in presents {
        let present = Present::new(p);
        result += present.paper_needed();
    }

    result
}

fn part_2(presents: &[&str]) -> u32 {
    let mut result = 0;
    for p in presents {
        let present = Present::new(p);
        result += present.ribbon_needed();
    }

    result
}
