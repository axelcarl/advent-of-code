pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn check_pair(curr: i32, prev: i32, positive: bool) -> bool {
    let diff = curr - prev;

    if diff.abs() > 3 || diff.abs() < 1 {
        return false;
    }

    if positive && curr < prev {
        return false;
    }

    if !positive && curr > prev {
        return false;
    }

    true
}

fn check_row(row: &str) -> i32 {
    if row == "" {
        return 0;
    }

    let mut iter = row.split(" ");
    let mut prev = iter.next().unwrap().parse::<i32>().unwrap();
    let mut curr = iter.next().unwrap().parse::<i32>().unwrap();

    if curr == prev {
        return 0;
    }

    let positive = curr > prev;

    for num in iter {
        if !check_pair(curr, prev, positive) {
            return 0;
        }

        prev = curr;
        curr = num.parse::<i32>().unwrap();
    }

    if !check_pair(curr, prev, positive) {
        return 0;
    }

    1
}

fn part_one(input: String) -> String {
    let safe = input.split("\n").map(|row| check_row(row)).sum::<i32>();

    safe.to_string()
}

fn get_level_variants(row: &str) -> Vec<String> {
    let levels: Vec<_> = row.split(" ").map(|l| l.to_string()).collect();
    let mut variants = vec![row.to_string()];

    for num in 0..levels.len() {
        let mut temp = levels.clone();
        temp.remove(num);

        let variant = temp.join(" ");

        variants.push(variant);
    }

    variants
}

fn part_two(input: String) -> String {
    let safe = input
        .split("\n")
        .map(|row| {
            let variants = get_level_variants(row);

            for variant in variants.iter() {
                if check_row(&variant) == 1 {
                    return 1;
                }
            }
            0
        })
        .sum::<i32>();

    safe.to_string()
}
