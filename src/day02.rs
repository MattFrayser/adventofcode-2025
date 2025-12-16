use std::collections::HashSet;
use std::fs;

/// Generate valid patterns and check number that could exit (O(digits))
/// Check every chunk size 'k' that divides the total length evenly.
/// Since patterns can overlap, found patterns are only marked once
pub fn solve() {
    let product_ranges = fs::read_to_string("inputs/day02.txt").expect("Missing input");
    let mut res = 0;

    for range in product_ranges.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        let s: u64 = start.parse().unwrap();
        let e: u64 = end.parse().unwrap();

        let mut found = HashSet::new();

        for len in start.len()..=end.len() {
            for k in 1..=(len / 2) {
                if len % k != 0 {
                    continue;
                }

                let repeats = len / k;
                let mult = calculate_multiplier(k as u32, repeats);

                let (start_x, end_x) = get_intersection(s, e, mult, k as u32);

                for id in start_x..=end_x {
                    let full_id = id * mult;
                    if found.insert(full_id) {
                        res += full_id;
                    }
                }
            }
        }
    }
    print!("{:?}", res);
}

/// Find the valid range of 'x' by intersecting two constraints:
///     1. Range Constraints
///         min_x = start / M
///         max_x max_x_range = end / M
///     2. Pattern size
///         min_k = 10^(k-1)
///         max_k = 10^k - 1
///
///     Valid range to search:
///         start = max(min_x, min_k)
///         end = min(max_x, max_k)
fn get_intersection(s: u64, e: u64, mult: u64, k: u32) -> (u64, u64) {
    let max_x = e / mult;
    let min_x = s.div_ceil(mult);

    let min_k = 10u64.pow(k - 1);
    let max_k = 10u64.pow(k) - 1;

    let start_x = min_x.max(min_k);
    let end_x = max_x.min(max_k);

    (start_x, end_x)
}

/// Generates the multiplier needed to repeat a k-digit block.
///
/// Calculates a geometric series of the block size (10^k).
/// Formula: M = (10^k)^0 + (10^k)^1 + ... + (10^k)^(reps-1)
///
/// k = number of digits
/// repetions = times to repeat k digits
///
/// EXAMPLES:
/// - k=2, reps=2: 1 + 100 = 101           | 12 * 101 = 1212        
/// - k=2, reps=3: 1 + 100 + 10000 = 10101 | 12 * 10101 = 121212
fn calculate_multiplier(k: u32, repetions: usize) -> u64 {
    let mut mult = 0;
    let shift = 10u64.pow(k);
    for _ in 0..repetions {
        mult = mult * shift + 1;
    }
    mult
}
