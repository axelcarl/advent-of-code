pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn get_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut row_one = vec![];
    let mut row_two = vec![];

    input.split("\n").for_each(|row| {
        let mut values = row.split("   ").map(|v| v.parse::<i32>().unwrap_or(0));
        row_one.push(values.next().unwrap_or(0));
        row_two.push(values.next().unwrap_or(0));
    });

    row_one.sort_by(|a, b| a.cmp(b));
    row_two.sort_by(|a, b| a.cmp(b));

    (row_one, row_two)
}

fn part_one(input: String) -> String {
    let (row_one, row_two) = get_lists(input);

    let mut iterable = row_two.iter();
    let distance: i32 = row_one
        .iter()
        .map(|row| {
            let l_id = iterable.next().unwrap();
            (l_id - row).abs()
        })
        .sum();

    format!("{distance}").to_string()
}

fn part_two(input: String) -> String {
    let (row_one, row_two) = get_lists(input);

    let mut last_num = -1;
    let mut last_val = -1;

    let total_occurrences: i32 = row_one
        .iter()
        .map(|num| {
            if *num == last_num {
                return last_val;
            }

            let occurrences = row_two.iter().fold(0, |acc, n| {
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
