use std::{collections::HashMap, fs};

/// Part 1 solution
/// Simple DFS traversal w/ memo
/// Preset Flags to true to count all paths
fn part1(adj_list: &HashMap<String, Vec<String>>) {
    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    let path_count = count_paths("you", adj_list, &mut memo, true, true);
    println!("Part 1: {}", path_count);
}

/// Part 2 solution
/// Simple DFS traversal w/ memo
fn part2(adj_list: &HashMap<String, Vec<String>>) {
    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    let path_count = count_paths("svr", adj_list, &mut memo, false, false);
    println!("Part 2: {}", path_count);
}

/// DFS Traverse of adj matrix
/// Memoize state (current and flags)
/// Use 'dac' and 'fft' flags, only increase count if both flags found on path
fn count_paths(
    curr: &str,
    adj_list: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), usize>,
    mut dac: bool,
    mut fft: bool,
) -> usize {
    if curr == "dac" {
        dac = true
    }
    if curr == "fft" {
        fft = true
    }

    if curr == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    let state = (curr.to_string(), dac, fft);

    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let mut path_count = 0;
    if let Some(values) = adj_list.get(curr) {
        for v in values {
            path_count += count_paths(v, adj_list, memo, dac, fft)
        }
    }

    memo.insert(state, path_count);
    path_count
}
pub fn solve() {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();

    let adj_list: HashMap<String, Vec<String>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split_whitespace();
            let key = parts.next().unwrap().trim_matches(':').to_string();
            let values: Vec<String> = parts.map(|s| s.to_string()).collect();
            (key, values)
        })
        .collect();

    //println!("HashMap: {:#?}", wire_adj_list);
    part1(&adj_list);
    part2(&adj_list);
}
