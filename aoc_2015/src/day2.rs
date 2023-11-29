#[derive(Debug, Clone, Copy)]
pub struct Present {
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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Present> {
    input.lines().map(|s| Present::new(s)).collect()
}

#[aoc(day2, part1)]
fn part_1(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.paper_needed()).sum()
}

#[aoc(day2, part2)]
fn part_2(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.ribbon_needed()).sum()
}
