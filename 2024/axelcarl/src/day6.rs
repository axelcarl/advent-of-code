use std::collections::{HashMap, HashSet};

pub fn solve(input: String) -> String {
    let output_one = part_one(input.clone());
    let output_two = part_two(input);

    format!("Part one: {output_one}, Part two: {output_two}").to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

impl Dir {
    fn ver(&self) -> bool {
        *self == Dir::UP || *self == Dir::DOWN
    }
    fn hor(&self) -> bool {
        *self == Dir::LEFT || *self == Dir::RIGHT
    }
}

fn part_one(input: String) -> String {
    let mut pos: i32 = 0;
    let mut dir = Dir::LEFT;
    let mut stones = vec![];
    let mut index = 0;

    let row = input.find("\n").unwrap();
    let map = input.trim().replace("\n", "");

    map.chars().for_each(|ch| {
        match ch {
            '<' => {
                pos = index;
                dir = Dir::LEFT
            }
            '^' => {
                pos = index;
                dir = Dir::UP
            }
            '>' => {
                pos = index;
                dir = Dir::RIGHT
            }
            'v' => {
                pos = index;
                dir = Dir::DOWN
            }
            '#' => stones.push(index),
            _ => (),
        };

        index += 1;
    });

    let mut char_map: Vec<char> = map.chars().collect();
    let len = char_map.len();

    while pos > 0 && pos < len as i32 {
        char_map[pos as usize] = 'X';

        let step = match dir {
            Dir::LEFT => -1,
            Dir::UP => -(row as i32),
            Dir::RIGHT => 1,
            Dir::DOWN => row as i32,
        };

        if pos as usize % row == row - 1 && step == 1 {
            break;
        } else if pos as usize % row == 0 && step == -1 {
            break;
        }

        if pos + step > len as i32 || pos + step < 0 {
            break;
        }

        let next = char_map[(pos + step) as usize];

        if next == '#' {
            // Rotate.
            dir = match dir {
                Dir::LEFT => Dir::UP,
                Dir::UP => Dir::RIGHT,
                Dir::RIGHT => Dir::DOWN,
                Dir::DOWN => Dir::LEFT,
            };

            continue;
        }

        pos += step;
    }

    let total: i32 = char_map
        .iter()
        .map(|ch| {
            if *ch == 'X' {
                return 1;
            } else {
                0
            }
        })
        .sum();

    format!("{total}")
}

fn part_two(input: String) -> String {
    let mut pos: i32 = 0;
    let mut dir = Dir::LEFT;
    let mut stones = vec![];
    let mut index = 0;

    let row = input.find("\n").unwrap();
    let map = input.trim().replace("\n", "");

    map.chars().for_each(|ch| {
        match ch {
            '<' => {
                pos = index;
                dir = Dir::LEFT
            }
            '^' => {
                pos = index;
                dir = Dir::UP
            }
            '>' => {
                pos = index;
                dir = Dir::RIGHT
            }
            'v' => {
                pos = index;
                dir = Dir::DOWN
            }
            '#' => stones.push(index),
            _ => (),
        };

        index += 1;
    });

    let char_map: Vec<char> = map.chars().collect();
    let len = char_map.len();
    let mut visited = HashMap::new();
    let mut reoccurring = HashSet::new();

    while pos > 0 && pos < len as i32 {
        let prev = visited.get(&pos);

        if prev.is_some() {
            let prev: &Dir = prev.unwrap();

            println!("{prev:?} {dir:?}");

            if prev.ver() && dir.hor() {
                reoccurring.insert(pos);
            }

            if prev.hor() && dir.ver() {
                reoccurring.insert(pos);
            }
        }

        visited.insert(pos, dir);

        let step = match dir {
            Dir::LEFT => -1,
            Dir::UP => -(row as i32),
            Dir::RIGHT => 1,
            Dir::DOWN => row as i32,
        };

        if pos as usize % row == row - 1 && step == 1 {
            break;
        } else if pos as usize % row == 0 && step == -1 {
            break;
        }

        if pos + step > len as i32 || pos + step < 0 {
            break;
        }

        let next = char_map[(pos + step) as usize];

        if next == '#' {
            // Rotate.
            dir = match dir {
                Dir::LEFT => Dir::UP,
                Dir::UP => Dir::RIGHT,
                Dir::RIGHT => Dir::DOWN,
                Dir::DOWN => Dir::LEFT,
            };

            continue;
        }

        pos += step;
    }

    let total = reoccurring.len();
    println!("{reoccurring:?}");
    format!("{total}")
}
