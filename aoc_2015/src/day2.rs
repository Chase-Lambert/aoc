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
        let mut side_lengths: Vec<u32> = vec![self.length, self.width, self.height];
        side_lengths.sort();

        side_lengths[0] * side_lengths[1]
    }

    fn smallest_perimeter(&self) -> u32 {
        let mut side_lengths: Vec<u32> = vec![self.length, self.width, self.height];
        side_lengths.sort();

        side_lengths[0] * 2 + side_lengths[1] * 2
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn paper_needed(&self) -> u32 {
        self.surface_area() + self.area_of_smallest_side()
    }

    fn ribbon_needed(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

fn part_1(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.paper_needed()).sum()
}

fn part_2(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.ribbon_needed()).sum()
}

pub fn day_02() {
    let input = include_str!("../resources/inputs/day_02.txt").trim();
    let presents: Vec<Present> = input.lines().map(|s| Present::new(s)).collect();

    println!("Day 2, Part 1: {:?}", part_1(&presents)); // 1598415
    println!("Day 2, Part 2: {:?}", part_2(&presents)); // 3812909
}
