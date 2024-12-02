pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn get_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    input.split("\n").for_each(|row| {
        let mut values = row.split("   ").map(|v| v.parse::<i32>().unwrap_or(0));
        left.push(values.next().unwrap_or(0));
        right.push(values.next().unwrap_or(0));
    });

    left.sort_by(|a, b| a.cmp(b));
    right.sort_by(|a, b| a.cmp(b));

    (left, right)
}

fn part_one(input: String) -> String {
    let (left, right) = get_lists(input);

    let mut iterable = right.iter();
    let distance: i32 = left
        .iter()
        .map(|row| {
            let l_id = iterable.next().unwrap();
            (l_id - row).abs()
        })
        .sum();

    format!("{distance}").to_string()
}

fn part_two(input: String) -> String {
    let (left, right) = get_lists(input);

    let mut last_num = -1;
    let mut last_val = -1;

    let total_occurrences: i32 = left
        .iter()
        .map(|num| {
            if *num == last_num {
                return last_val;
            }

            let occurrences = right.iter().fold(0, |acc, n| {
                if *n == *num {
                    return acc + 1;
                }

                acc
            });

            last_num = *num;
            last_val = *num * occurrences;

            last_val
        })
        .sum();

    total_occurrences.to_string()
}
