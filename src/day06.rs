pub fn solve() {
    let inputs = std::fs::read_to_string("inputs/day06.txt").unwrap();

    part1(&inputs);
    part2(&inputs);
}
fn part1(inputs: &str) {
    let worksheet: Vec<Vec<&str>> = inputs
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut res = 0;
    for col in 0..worksheet[0].len() {
        let op = worksheet.last().unwrap()[col];

        let nums: Vec<u64> = (0..worksheet.len() - 1)
            .filter_map(|r| worksheet.get(r).and_then(|row| row.get(col)))
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        match op {
            "+" => res += nums.iter().sum::<u64>(),
            "*" => res += nums.iter().product::<u64>(),
            _ => panic!("Unknown operator: {}", op),
        }
    }

    println!("{res}");
}

fn part2(inputs: &str) {
    let worksheet: Vec<&str> = inputs.lines().collect();

    let mut columns: Vec<String> = (0..worksheet[0].len())
        .map(|i| {
            worksheet
                .iter()
                .filter_map(|line| line.chars().nth(i))
                .collect()
        })
        .collect();

    columns.reverse();

    let mut res: u64 = 0;
    for col in columns.split(|col| col.trim().is_empty()) {
        let operator = col[col.len() - 1].chars().last().unwrap();

        let nums: Vec<u64> = col
            .iter()
            .map(|s| {
                let num: String = s.chars().take(s.len() - 1).collect();
                num.trim().parse::<u64>().unwrap()
            })
            .collect();

        match operator {
            '+' => res += nums.iter().sum::<u64>(),
            '*' => res += nums.iter().product::<u64>(),
            _ => panic!("non operator"),
        }
    }

    println!("{res}");
}
