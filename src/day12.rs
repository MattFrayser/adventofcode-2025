use std::fs;

#[derive(Debug)]
struct Shape {
    /// Rotations and flips
    shape: Vec<Vec<bool>>,
}

impl Shape {
    fn area(&self) -> usize {
        self.shape.iter().flatten().filter(|&&b| b).count()
    }
}

impl From<&&str> for Shape {
    fn from(value: &&str) -> Self {
        let mut lines = value.lines();
        // Skip first line.
        lines.next();
        let shape: Vec<Vec<bool>> = lines
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        Self { shape }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (dims, shapes) = value.split_once(": ").unwrap();
        let (width, height) = dims.split_once("x").unwrap();

        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            shapes: shapes
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

/// All regions have enough 3x3 to fit all shapes or will be too small
/// Check sanity check
/// Count regions that have enough 3x3 areas
fn part1(regions: &[Region]) {
    let res = regions
        .iter()
        .filter(|region| {
            let available_grids = (region.width / 3) * (region.height / 3);
            let shapes_needed = region.shapes.iter().sum();
            available_grids >= shapes_needed
        })
        .count();

    println!("Part 1: {}", res);
}

/// All shapes are 3x3 grid.
/// - Fast fail: Total shape area exceeds region area (impossible)
/// - Easy: Enough 3x3 slots for all shapes (each shape fits in 3x3)
/// - Ambiguous: Enough area but not enough 3x3 slots (requires overlap/packing)
fn sanity(shapes: &[Shape], regions: &[Region]) {
    let mut easy = 0;
    let mut fast_fail = 0;
    let mut ambiguous = 0;

    for region in regions.iter() {
        let available_grid = (region.width / 3) * (region.height / 3);
        let shapes_needed: usize = region.shapes.iter().sum();

        let region_area = region.width * region.height;
        let shapes_area: usize = region
            .shapes
            .iter()
            .enumerate()
            .map(|(shape_idx, &count)| count * shapes[shape_idx].area())
            .sum();

        if shapes_area > region_area {
            fast_fail += 1;
        } else if available_grid >= shapes_needed {
            easy += 1;
        } else {
            ambiguous += 1;
        }
    }

    println!(
        "[Sanity Check] easy: {}, fast_fail: {}, ambiguous?: {}",
        easy, fast_fail, ambiguous
    );
}

pub fn solve() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    let sections: Vec<_> = input.trim().split("\n\n").collect();

    let shapes: Vec<Shape> = sections[..sections.len() - 1]
        .iter()
        .map(Shape::from)
        .collect();

    let regions: Vec<Region> = sections[sections.len() - 1]
        .lines()
        .map(Region::from)
        .collect();

    //println!("Shapes: {:#?}", shapes);
    //println!("Regions: {:#?}", regions);
    sanity(&shapes, &regions);
    part1(&regions);
}
