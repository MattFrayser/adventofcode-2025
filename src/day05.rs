pub fn solve() {
    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();

    let (raw_ranges, raw_ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = raw_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.trim().split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let fresh_ids: Vec<u64> = raw_ids
        .lines()
        .map(|line| line.parse().expect("Not a number"))
        .collect();

    let merged_ranges = merge_ranges(ranges);
    let mut p1_res = 0;
    for id in fresh_ids {
        if merged_ranges.iter().any(|(s, e)| &id >= s && &id <= e) {
            p1_res += 1
        }
    }
    println!("Part 1: {p1_res}");

    let mut p2_res = 0;
    for (start, end) in merged_ranges {
        p2_res += end - start + 1;
    }
    println!("Part 2: {p2_res}");
}

/// Merge overlaping ranges into singe tuple
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for &(next_start, next_end) in ranges.iter().skip(1) {
        if cur_end >= next_start {
            cur_end = next_end.max(cur_end);
        } else {
            merged_ranges.push((cur_start, cur_end));
            cur_start = next_start;
            cur_end = next_end;
        }
    }
    merged_ranges.push((cur_start, cur_end));

    merged_ranges
}
