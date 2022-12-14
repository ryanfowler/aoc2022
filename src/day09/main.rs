use std::collections::HashSet;

fn main() {
    println!("Day 09");
    println!("======");

    let input = include_str!("./input.txt");

    let ts = std::time::Instant::now();
    let ans1 = tail_positions(input, 1);
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = tail_positions(input, 9);
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn tail_positions(input: &str, num_tails: usize) -> usize {
    assert!(num_tails >= 1);
    let mut set = HashSet::new();

    let mut head = (0, 0);
    let mut tails = vec![(0, 0); num_tails];
    for (direction, distance) in input.lines().map(parse_command) {
        for _ in 0..distance {
            head.0 += direction.0;
            head.1 += direction.1;
            let mut prev = head;
            for tail in tails.iter_mut() {
                let x: i32 = prev.0 - tail.0;
                let y: i32 = prev.1 - tail.1;
                if x.abs() > 1 {
                    tail.0 += x.signum();
                    if y.abs() > 0 {
                        tail.1 += y.signum();
                    }
                } else if y.abs() > 1 {
                    tail.1 += y.signum();
                    if x.abs() > 0 {
                        tail.0 += x.signum();
                    }
                }
                prev = *tail;
            }
            set.insert(tails[tails.len() - 1]);
        }
    }

    set.len()
}

fn parse_command(s: &str) -> ((i32, i32), usize) {
    let (dir, num) = s.split_once(' ').unwrap();
    let dir = match dir {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => (0, 0),
    };
    (dir, num.parse().unwrap())
}
