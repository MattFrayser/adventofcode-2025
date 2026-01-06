use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

pub struct Edge {
    dist: i64,
    u: usize,
    v: usize,
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }

        true
    }
}

pub fn solve() {
    let input = fs::read_to_string("inputs/day08.txt").unwrap();

    let map: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut pos = line.split(",");

            Point {
                x: pos.next().unwrap().trim().parse().unwrap(),
                y: pos.next().unwrap().trim().parse().unwrap(),
                z: pos.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    let n = map.len();
    let mut edges = Vec::with_capacity(n * n / 2);

    for (i, p1) in map.iter().enumerate() {
        for (j, p2) in map.iter().enumerate().skip(i + 1) {
            edges.push(Edge {
                dist: p1.dist(p2),
                u: i,
                v: j,
            })
        }
    }
    edges.sort_unstable_by_key(|e| e.dist);

    part1(n, &edges);
    part2(&map, &edges);
}

/// Largest 3 cicuits in set amount
fn part1(n: usize, edges: &[Edge]) {
    let mut dsu = DisjointSet::new(n);

    for edge in edges.iter().take(1000) {
        dsu.union(edge.u, edge.v);
    }

    let mut circuits = Vec::new();
    for i in 0..n {
        if dsu.parent[i] == i {
            circuits.push(dsu.size[i]);
        }
    }

    circuits.sort_unstable();
    let res: usize = circuits.iter().rev().take(3).product();
    println!("Part 1: {res}");
}

/// Last points added to circuits x's product
fn part2(map: &[Point], edges: &[Edge]) {
    let n = map.len();
    let mut dsu = DisjointSet::new(n);

    let mut merges = 0;
    for edge in edges.iter() {
        if dsu.union(edge.u, edge.v) {
            merges += 1;
            if merges == n - 1 {
                let point_u = map[edge.u];
                let point_v = map[edge.v];
                let res = point_u.x * point_v.x;
                println!("Part 2: {res}");
                break;
            }
        }
    }
}
