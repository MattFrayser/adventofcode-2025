use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
pub struct Machine {
    target: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

/// Part 1 Solution
///
/// BFS Traversal
/// Returns minimum number of XOR operations to turn 0 into target
fn part1(machines: &[Machine]) {
    let mut res = 0;

    for machine in machines {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0, 0));
        visited.insert(0);
        let mut button_presses = 0;

        while let Some((curr, count)) = queue.pop_front() {
            if curr == machine.target {
                button_presses = count;
                break;
            }

            for button in &machine.buttons {
                let next = curr ^ button;
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, count + 1));
                }
            }
        }

        res += button_presses;
    }

    println!("Part 1: {res}");
}

/// Part 2 Solution
///
/// Returns minimum cost to reduce all joltages to 0
///
/// Approach
/// 1. look at the Least Significant Bit (parity) of the current numbers.
/// 2. find a combination of buttons that "fixes" the parity (makes all numbers even)
/// 3. Once all numbers are even, we divide by 2 (right shift), effectively moving to the next bit plane
/// 4. Recurse until the state is all zeros
fn part2(machines: &[Machine]) {
    let mut res = 0;
    for machine in machines {
        let rows = machine.joltages.len();

        let patterns = generate_patterns(&machine.buttons, rows);
        //println!("{:#?}", patterns);

        let mut memo = HashMap::new();
        if let Some(cost) = min_button_presses(&machine.joltages, &patterns, &mut memo) {
            res += cost;
        }
    }
    println!("Part 2: {}", res);
}

/// Brute force every possible pattern of button presses
/// Check what effect each pattern has on voltage
///
/// # Returns
/// HashMap:
/// * Key (u32): Bitmask Pattern (ex: bits 0 and 2 set means move fixes odd nums at index 0 and 2)
/// * value (Vec): List of moves (sorted least presses)
fn generate_patterns(buttons: &[u32], rows: usize) -> HashMap<u32, Vec<(Vec<u32>, u64)>> {
    let n = buttons.len();
    let total_subsets: usize = 1 << n;
    let mut patterns: HashMap<u32, Vec<(Vec<u32>, u64)>> = HashMap::new();

    for combo in 0..total_subsets {
        // Accumulator for actual voltage drop caused by combo
        let mut impact = vec![0; rows];
        let mut count = 0;

        for i in 0..n {
            // Is i-th bit 'on' -> Calc its effect
            if (combo & (1 << i)) != 0 {
                count += 1;
                let btn = buttons[i];

                for r in 0..rows {
                    // Does button effect row?
                    if (btn & (1 << r)) != 0 {
                        impact[r] += 1;
                    }
                }
            }
        }

        // Compress integer changes into binary ID
        // Iteratate impact vec to find which rows get 'fliped' (odd -> even)
        // Only care about odd number as even dont 'flip' switch (odd - even = odd)
        let mut parity = 0;
        for (r, &val) in impact.iter().enumerate() {
            if val % 2 != 0 {
                parity |= 1 << r;
            }
        }

        patterns.entry(parity).or_default().push((impact, count));
    }
    // Pre sort by cost (greedy to find min cost faster)
    for list in patterns.values_mut() {
        list.sort_by_key(|(_vec, count)| *count);
    }
    patterns
}

/// Recursive function to find minimum button presses
///
/// Calculate current parity
/// Find pattern to make parity all even
/// divide by 2, recurse
fn min_button_presses(
    goal: &[u32],
    patterns: &HashMap<u32, Vec<(Vec<u32>, u64)>>,
    memo: &mut HashMap<Vec<u32>, Option<u64>>,
) -> Option<u64> {
    if let Some(&res) = memo.get(goal) {
        return res;
    }

    // Base case: All zeros, Calculate parity
    let mut current_parity = 0;
    let mut all_zero = true;
    for (i, &x) in goal.iter().enumerate() {
        if x != 0 {
            all_zero = false;
        }
        if x % 2 != 0 {
            current_parity |= 1 << i;
        }
    }

    if all_zero {
        return Some(0);
    };

    let best_cost = patterns.get(&current_parity).and_then(|candidates| {
        candidates
            .iter()
            .filter_map(|(effect, cost)| {
                // (Goal - Effect) must be non-negative
                if goal.iter().zip(effect).all(|(g, e)| g >= e) {
                    // next: (Goal - Effect) / 2
                    let next_goal: Vec<u32> =
                        goal.iter().zip(effect).map(|(g, e)| (g - e) / 2).collect();

                    if let Some(total_cost) = min_button_presses(&next_goal, patterns, memo) {
                        // Total = curr cost + 2 * next layer cost
                        return Some(cost + 2 * total_cost);
                    }
                }
                None
            })
            .min()
    });

    memo.insert(goal.to_vec(), best_cost);
    best_cost
}

pub fn solve() {
    let inputs = fs::read_to_string("inputs/day10.txt").unwrap();

    let machines: Vec<Machine> = inputs
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let target = parts[0][1..]
                .bytes()
                .enumerate()
                .fold(0, |acc, (i, b)| acc | (u32::from(b == b'#') << i));

            let buttons: Vec<u32> = parts[1..parts.len() - 1]
                .iter()
                .map(|b| {
                    b.split(|c: char| !c.is_numeric())
                        .filter_map(|b| b.parse::<u32>().ok())
                        .fold(0, |acc, idx| acc | (1 << idx))
                })
                .collect();

            let joltages: Vec<u32> = parts
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            Machine {
                target,
                buttons,
                joltages,
            }
        })
        .collect();

    part1(&machines);
    part2(&machines);
}
