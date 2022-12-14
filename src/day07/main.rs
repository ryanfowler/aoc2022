use std::collections::HashMap;

fn main() {
    println!("Day 07");
    println!("======");

    let input = include_str!("./input.txt");
    let sizes = parse_sizes(input);

    let ts = std::time::Instant::now();
    let ans1: i32 = sizes
        .iter()
        .filter(|(_, &v)| v <= 100_000)
        .map(|(_, &v)| v)
        .sum();
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let to_free = *sizes.get("/").unwrap() - 40_000_000;
    let ans2 = sizes
        .iter()
        .filter(|(_, &v)| v >= to_free)
        .fold(i32::MAX, |min, (_, &v)| std::cmp::min(min, v));
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn parse_sizes(s: &str) -> HashMap<String, i32> {
    let mut sizes = HashMap::new();

    let mut path = Vec::new();
    for line in s.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        // Assume that directories are only ever listed once; this lets us add
        // any file that we visit to the hashmap using the current path.
        match (split[0], split[1]) {
            ("$", "cd") => {
                if split[2] == ".." {
                    path.pop();
                } else {
                    path.push(split[2]);
                }
            }
            ("$", _) | ("dir", _) => {}
            (size, _) => {
                if let Ok(size) = size.parse::<i32>() {
                    for i in 0..path.len() + 1 {
                        *sizes.entry(path[..i].join("/")).or_insert(0) += size;
                    }
                }
            }
        }
    }

    sizes
}
