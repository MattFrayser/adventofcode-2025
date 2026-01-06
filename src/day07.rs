use std::{collections::HashMap, fs};

/// DFS search for ^ from S
pub fn solve() {
    let input = fs::read_to_string("inputs/day07.txt").unwrap();

    let s = input.lines().next().unwrap().find("S").unwrap();
    let manifold: Vec<Vec<char>> = input.lines().skip(1).map(|s| s.chars().collect()).collect();

    let rows = manifold.len();
    let cols = manifold[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let p1_res = part1_dfs(0, s as isize, &manifold, &mut visited);
    println!("Part1: {:?}", p1_res);

    let mut p2_memo: HashMap<(isize, isize), usize> = HashMap::new();
    let p2_res = part2_dfs(0, s as isize, &manifold, &mut p2_memo);
    println!("Part2: {:?}", p2_res);
}

/// Split left AND right on each ^
/// Return count of each ^ encountered
fn part1_dfs(r: isize, c: isize, manifold: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> usize {
    let rows = manifold.len() as isize;
    let cols = manifold[0].len() as isize;

    if r < 0 || c < 0 || r >= rows || c >= cols {
        return 0;
    }
    if visited[r as usize][c as usize] {
        return 0;
    }
    visited[r as usize][c as usize] = true;

    let mut count = 0;
    if manifold[r as usize][c as usize] == '^' {
        count = 1;
        count += part1_dfs(r + 1, c - 1, manifold, visited);
        count += part1_dfs(r + 1, c + 1, manifold, visited);
    } else {
        count += part1_dfs(r + 1, c, manifold, visited);
    }

    count
}

/// Split left OR right on ^
/// Return number of possible path to bottom
fn part2_dfs(
    r: isize,
    c: isize,
    manifold: &Vec<Vec<char>>,
    memo: &mut HashMap<(isize, isize), usize>,
) -> usize {
    let rows = manifold.len() as isize;
    let cols = manifold[0].len() as isize;

    if r < 0 || c < 0 || r >= rows || c >= cols {
        return 0;
    }
    if r == rows - 1 {
        return 1;
    }
    if memo.contains_key(&(r, c)) {
        return memo[&(r, c)];
    }

    let mut count = 0;
    if manifold[r as usize][c as usize] == '^' {
        count += part2_dfs(r + 1, c - 1, manifold, memo);
        count += part2_dfs(r + 1, c + 1, manifold, memo);
    } else {
        count += part2_dfs(r + 1, c, manifold, memo);
    }
    memo.insert((r, c), count);

    count
}
