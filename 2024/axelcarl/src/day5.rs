use std::collections::HashMap;

pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

fn part_one(input: String) -> String {
    let mut iter = input.split("\n\n");

    let raw_rules = iter.next().unwrap().trim();
    let raw_orders = iter.next().unwrap().trim();

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    raw_rules.split("\n").for_each(|line| {
        let mut iter = line.split("|");

        let value: usize = iter.next().unwrap().parse().unwrap();
        let key: usize = iter.next().unwrap().parse().unwrap();

        rules.entry(key).or_insert_with(Vec::new).push(value);
    });

    let total: usize = raw_orders
        .split("\n")
        .map(|line| {
            let orders: Vec<usize> = line.split(",").map(|o| o.parse().unwrap()).collect();
            let value = orders[(orders.len() - 1) / 2];
            let mut seen = vec![];

            for o in orders.iter() {
                seen.push(o);
                let rules = rules.get(&o);

                if rules.is_none() {
                    continue;
                }

                for r in rules.unwrap().iter() {
                    if !seen.contains(&r) && orders.contains(&r) {
                        return 0;
                    }
                }
            }

            value
        })
        .sum();

    format!("{total}").to_string()
}

fn shuffle(
    order: usize,
    rules: HashMap<usize, Vec<usize>>,
    orders: Vec<usize>,
    new: Vec<usize>,
) -> usize {
    let mut value = order;
    let mut overrule = None;

    if !rules.contains_key(&order) {
        return value;
    }

    for rule in rules[&order].clone() {
        if orders.contains(&rule) && !new.contains(&rule) {
            overrule = Some(rule);
            break;
        }
    }

    if overrule.is_some() {
        value = shuffle(overrule.unwrap(), rules.clone(), orders, new);
    }

    value
}

fn part_two(input: String) -> String {
    let mut iter = input.split("\n\n");

    let raw_rules = iter.next().unwrap().trim();
    let raw_orders = iter.next().unwrap().trim();

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    raw_rules.split("\n").for_each(|line| {
        let mut iter = line.split("|");

        let value: usize = iter.next().unwrap().parse().unwrap();
        let key: usize = iter.next().unwrap().parse().unwrap();

        rules.entry(key).or_insert_with(Vec::new).push(value);
    });

    let total: usize = raw_orders
        .split("\n")
        .map(|line| {
            let orders: Vec<usize> = line.split(",").map(|o| o.parse().unwrap()).collect();
            let mut new: Vec<usize> = vec![];

            for order in orders.clone() {
                while !new.contains(&order) {
                    new.push(shuffle(order, rules.clone(), orders.clone(), new.clone()));
                }
            }

            if orders != new {
                return new[(new.len() - 1) / 2];
            }
            0
        })
        .sum();

    format!("{total}").to_string()
}
