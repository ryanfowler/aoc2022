fn main() {
    println!("Day 15");
    println!("======");

    let input = include_str!("./input.txt").trim_end();
    let data = input
        .lines()
        .map(parse_sensor)
        .map(|(sensor, beacon)| (sensor, distance(sensor, beacon)))
        .collect::<Vec<_>>();

    let ts = std::time::Instant::now();
    let ans1 = part1(&data);
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = part2(&data);
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn part1(input: &[((i32, i32), i32)]) -> i32 {
    let mut ranges = input
        .iter()
        .filter_map(|&(sensor, distance)| range_not_at(2_000_000, (sensor, distance)))
        .collect::<Vec<_>>();
    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].1 >= ranges[i + 1].0 {
            ranges[i].1 = ranges[i + 1].1.max(ranges[i].1);
            ranges.remove(i + 1);
            continue;
        }
        i += 1;
    }

    ranges.iter().map(|&v| v.1 - v.0).sum()
}

fn part2(input: &[((i32, i32), i32)]) -> i64 {
    const MIN: i32 = 0;
    const MAX: i32 = 4_000_000;
    let allowed = MIN..=MAX;

    for &((sx, sy), distance) in input {
        let edge = distance + 1;
        for ex in -edge..=edge {
            let x = sx + ex;
            if !allowed.contains(&x) {
                continue;
            }
            let dy = edge - ex;
            for y in [sy + dy, sy - dy] {
                if !allowed.contains(&y) {
                    continue;
                }
                if input
                    .iter()
                    .all(|&((bx, by), d)| (bx - x).abs() + (by - y).abs() >= d)
                {
                    return x as i64 * MAX as i64 + y as i64;
                }
            }
        }
    }

    0
}

fn range_not_at(y: i32, data: ((i32, i32), i32)) -> Option<(i32, i32)> {
    let ((sx, sy), distance) = data;
    let n = (y - sy).abs();
    if n < distance {
        let k = distance - n;
        Some((sx - k, sx + k))
    } else {
        None
    }
}

fn parse_sensor(input: &str) -> ((i32, i32), (i32, i32)) {
    let (left, right) = input[10..].split_once(": closest beacon is at ").unwrap();
    (parse_coords(left), parse_coords(right))
}

fn parse_coords(input: &str) -> (i32, i32) {
    let (left, right) = input.split_once(", ").unwrap();
    (left[2..].parse().unwrap(), right[2..].parse().unwrap())
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
