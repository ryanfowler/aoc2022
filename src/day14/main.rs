use std::collections::HashSet;

fn main() {
    println!("Day 14");
    println!("======");

    let input = include_str!("./input.txt").trim_end();
    let set = parse_input(input);

    let ts = std::time::Instant::now();
    let ans1 = calc_sand(set.clone(), false);
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = calc_sand(set, true);
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    for line in input.lines() {
        let mut prev: Option<(i32, i32)> = None;
        for pair in line.split(" -> ") {
            let (left, right) = pair.split_once(',').unwrap();
            let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
            if let Some(prev) = prev {
                let (xd, yd): (i32, i32) = (left - prev.0, right - prev.1);
                for i in 0..=xd.abs().max(yd.abs()) {
                    let point = (prev.0 + i * xd.signum(), prev.1 + i * yd.signum());
                    set.insert(point);
                }
            }
            prev = Some((left, right));
        }
    }
    set
}

fn calc_sand(mut set: HashSet<(i32, i32)>, is_floor: bool) -> i32 {
    let max_y = set
        .iter()
        .fold(i32::MIN, |acc, v| if v.1 > acc { v.1 } else { acc });
    let floor = max_y + 2;

    let mut path = Vec::new();
    for i in 0.. {
        let mut sand = path.pop().unwrap_or((500, 0));
        loop {
            if !is_floor && sand.1 > max_y {
                return i;
            }
            if set.get(&(sand.0, sand.1 + 1)).is_none() {
                sand = (sand.0, sand.1 + 1);
            } else if set.get(&(sand.0 - 1, sand.1 + 1)).is_none() {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if set.get(&(sand.0 + 1, sand.1 + 1)).is_none() {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                path.pop();
                break;
            }
            if is_floor && sand.1 + 1 == floor {
                path.pop();
                break;
            }
            path.push(sand);
        }
        if sand == (500, 0) {
            return i + 1;
        }
        set.insert(sand);
    }
    0
}
