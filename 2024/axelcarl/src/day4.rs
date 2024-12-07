pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn to_index(num: i32) -> Option<usize> {
    if num < 0 {
        return None;
    }

    Some(num as usize)
}

fn get_xmas_points(index: usize, map: &Vec<char>, row: i32) -> i32 {
    let directions = vec![1, -1, row, -row, row + 1, row - 1, -row + 1, -row - 1];

    directions
        .iter()
        .map(|dir| {
            let word: String = vec![1, 2, 3]
                .iter()
                .map(|i| {
                    let wrapped_idx = to_index(index as i32 + (dir * i));

                    if wrapped_idx.is_none() {
                        return '*';
                    }

                    let idx = wrapped_idx.unwrap();

                    // Boundary checking.
                    if idx >= map.len() {
                        return '*';
                    }

                    // Check modulus.
                    let col_pos: i32 = (idx as i32) % row;
                    let col_start: i32 = (index as i32) % row;

                    if (col_pos - col_start).abs() > 3 {
                        return '*';
                    }

                    map[idx]
                })
                .collect();

            if word == "MAS" {
                return 1;
            }

            0
        })
        .sum()
}

fn part_one(input: String) -> String {
    let mut index = 0;
    let mut total = 0;

    let row = input.find("\n").unwrap();
    let map = input.trim().replace("\n", "");
    let chars: Vec<_> = map.chars().collect();
    map.chars().for_each(|ch| {
        if ch == 'X' {
            total += get_xmas_points(index, &chars, row as i32);
        }

        index += 1;
    });

    format!("{total}")
}

fn get_crossmas_points(idx: usize, map: &Vec<char>, row: usize) -> i32 {
    let tl = map[idx - row - 1];
    let tr = map[idx - row + 1];
    let bl = map[idx + row - 1];
    let br = map[idx + row + 1];

    let word: String = vec![tl, tr, bl, br].iter().collect();

    match &word[0..word.len()] {
        "MMSS" => 1,
        "SSMM" => 1,
        "SMSM" => 1,
        "MSMS" => 1,
        _ => 0,
    }
}

fn mirror(map: String) -> String {
    map.trim()
        .split("\n")
        .map(|line| line.chars().rev().collect::<String>())
        .collect()
}

fn part_two(input: String) -> String {
    let mut total = 0;

    let row = input.find("\n").unwrap();
    let map = mirror(input);
    let chars: Vec<_> = map.chars().collect();

    for i in row..chars.len() - row {
        if i % row == 0 || i % row == row - 1 {
            continue;
        }

        if chars[i] == 'A' {
            total += get_crossmas_points(i, &chars, row as usize);
        }
    }

    format!("{total}")
}
