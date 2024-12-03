pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn mul(mul_str: String) -> i32 {
    let values: Vec<_> = mul_str.split(",").collect();

    values.get(0).unwrap_or(&"0").parse::<i32>().unwrap_or(0)
        * values.get(1).unwrap_or(&"0").parse::<i32>().unwrap_or(0)
}

fn part_one(input: String) -> String {
    let mut match_str = "".to_string();
    let mut mul_str = "".to_string();
    let mut is_mul = false;
    let mut total = 0;

    input.trim().chars().into_iter().for_each(|c| {
        if c == ')' && is_mul {
            // Contents collected.
            total += mul(mul_str.clone());
            is_mul = false;
        }

        match_str.push(c);
        mul_str.push(c);

        if match_str == "mul(" {
            is_mul = true;
            mul_str = "".to_string();
        }

        if match_str.len() > 3 {
            // Clear first char.
            match_str.remove(0);
        }
    });

    total.to_string()
}

fn part_two(input: String) -> String {
    let mut parsed = "".to_string();
    let mut mul_str = "".to_string();
    let mut is_mul = false;
    let mut total = 0;
    let mut should_do = true;

    input.trim().chars().into_iter().for_each(|c| {
        if c == ')' {
            if &parsed[parsed.len() - 3..] == "do(" {
                // Handle do.
                should_do = true;
            } else if &parsed[parsed.len() - 6..] == "don't(" {
                // Handle don't.
                should_do = false;
            }

            if is_mul && should_do {
                // Contents collected.
                total += mul(mul_str.clone());
                is_mul = false;
            }
        }

        parsed.push(c);
        mul_str.push(c);

        if c == '(' {
            if &parsed[parsed.len() - 4..] == "mul(" {
                // Handle mul.
                is_mul = true;
                mul_str = "".to_string();
            }
        }
    });

    total.to_string()
}
