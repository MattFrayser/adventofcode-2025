/// Day 3 Solution
///
/// Greedy approach using stack
/// * Calc skips (row len - digits allowed)
/// * While skips are available if current number > stack top -> pop stack  
/// truncate stack @ end to account for unused skips
pub fn solve() {
    let inputs = std::fs::read_to_string("inputs/day03.txt").unwrap();

    let mut res = 0;
    let mut keep = Vec::with_capacity(12);

    for bank in inputs.lines() {
        if bank.len() < 2 {
            eprintln!("Error: input row is too small");
        }

        keep.clear();

        let digits = bank.as_bytes();
        let mut skips = digits.len() - 12;

        for b in digits {
            let digit = (b - b'0') as u32;

            while skips > 0 && !keep.is_empty() && *keep.last().unwrap() < digit {
                keep.pop();
                skips -= 1;
            }
            keep.push(digit);
        }

        keep.truncate(12);

        res += keep.iter().fold(0, |acc, &x| acc * 10 + x as u64);
    }
    println!();
    println!("{}", res);
}
