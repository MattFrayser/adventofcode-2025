use std::collections::VecDeque;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();

    #[rustfmt::skip]
    let mut grid: Vec<Vec<u8>> = input
        .lines() 
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    #[rustfmt::skip]
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let should_remove = |r: usize, c: usize, grid: &Vec<Vec<u8>>| -> bool {
        if grid[r][c] != b'@' {
            return false;
        }

        let count = directions
            .iter()
            .map(|(dr, dc)| (r as isize + dr, c as isize + dc))
            .filter(|&(nr, nc)| nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize)
            .filter(|&(nr, nc)| grid[nr as usize][nc as usize] == b'@')
            .count();

        count < 4
    };

    let mut queue = VecDeque::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if should_remove(r, c, &grid) {
                queue.push_back((r, c));
            }
        }
    }

    let mut res = 0;

    while let Some((r, c)) = queue.pop_front() {
        if grid[r][c] == b'.' {
            continue;
        }

        grid[r][c] = b'.';
        res += 1;

        for (dr, dc) in directions {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0
                && nr < rows as isize
                && nc >= 0
                && nc < cols as isize
                && should_remove(nr as usize, nc as usize, &grid)
            {
                queue.push_back((nr as usize, nc as usize));
            }
        }
    }

    println!("{}", res);
}
