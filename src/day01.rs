use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/day01.txt").unwrap();

    let mut position = 50;
    let mut zeros = 0;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let (direction, number) = line.split_at(1);
        let n: i32 = number.parse().unwrap();

        match direction {
            "L" => {
                // Start @ zero does not count as a zero
                let dist_to_zero = if position == 0 { 100 } else { position };

                if n >= dist_to_zero {
                    // +1 to account for first zero
                    zeros += (n - dist_to_zero).div_euclid(100) + 1;
                }

                position = (position - n).rem_euclid(100);
            }
            "R" => {
                zeros += (position + n).div_euclid(100);
                position = (position + n).rem_euclid(100);
            }
            _ => panic!(),
        };

        // debug
        // println!(
        //     "{:?} {:?} -> {:?} (zeros: {:?})",
        //     direction, number, position, zeros
        // );
    }

    println!("password: {zeros}");
}
