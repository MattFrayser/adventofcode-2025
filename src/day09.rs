use std::fs;

pub struct Point {
    x: i32,
    y: i32,
}

/// Valid row points for each col
pub struct Polygon {
    y_min: i32,
    rows: Vec<Option<(i32, i32)>>,
}

impl Polygon {
    /// Search Optimized Polygon
    /// Identifies Vertical walls and calulates horizontal span
    /// Span (start, end) for every y coord
    fn new(grid: &[Point]) -> Self {
        let min_y = grid.iter().map(|p| p.y).min().unwrap();
        let max_y = grid.iter().map(|p| p.y).max().unwrap();
        let height = (max_y - min_y + 1) as usize;

        let mut rows: Vec<Vec<i32>> = vec![Vec::new(); height];

        for i in 0..grid.len() {
            let p1 = &grid[i];
            let p2 = &grid[(i + 1) % grid.len()];

            if p1.x == p2.x {
                let y0 = p1.y.min(p2.y);
                let y1 = p1.y.max(p2.y);
                for y in y0..=y1 {
                    rows[(y - min_y) as usize].push(p1.x);
                }
            }
        }

        let rows = rows
            .into_iter()
            .map(|mut x| {
                if x.is_empty() {
                    None
                } else {
                    x.sort_unstable();
                    Some((x[0], *x.last().unwrap()))
                }
            })
            .collect();

        Self { rows, y_min: min_y }
    }

    /// Return true if rectangle is entirely within polygon
    /// Checks vertical row of rectangle against polygons valid row spans
    fn contains_rect(&self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> bool {
        let y0 = y_min - self.y_min;
        let y1 = y_max - self.y_min;

        for y in y0..=y1 {
            match self.rows.get(y as usize).and_then(|v| *v) {
                Some((start, end)) => {
                    if x_min < start || x_max > end {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}

fn part1(grid: &[Point]) {
    let mut max_area = 0;
    for (i, p1) in grid.iter().enumerate() {
        for p2 in grid.iter().skip(i + 1) {
            let x_min = p1.x.min(p2.x);
            let x_max = p1.x.max(p2.x);
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);

            let area = ((x_max - x_min + 1) as u64) * ((y_max - y_min + 1) as u64);

            max_area = max_area.max(area);
        }
    }

    println!("Part 1: {max_area}");
}

fn part2(grid: &[Point]) {
    let polygon = Polygon::new(grid);
    let mut max_area: u64 = 0;
    for (i, p1) in grid.iter().enumerate() {
        for p2 in grid.iter().skip(i + 1) {
            let x_min = p1.x.min(p2.x);
            let x_max = p1.x.max(p2.x);
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);

            let area = ((x_max - x_min + 1) as u64) * ((y_max - y_min + 1) as u64);

            if area <= max_area {
                continue;
            }

            if polygon.contains_rect(x_min, x_max, y_min, y_max) {
                max_area = area;
            }
        }
    }
    println!("Part 2: {}", max_area);
}

pub fn solve() {
    let inputs = fs::read_to_string("inputs/day09.txt").expect("file");

    let grid: Vec<Point> = inputs
        .lines()
        .map(|l| {
            let (c, r) = l.split_once(",").expect(",");
            Point {
                x: c.parse().unwrap(),
                y: r.parse().unwrap(),
            }
        })
        .collect();

    part1(&grid);
    part2(&grid);
}
