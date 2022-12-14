fn main() {
    println!("Day 04");
    println!("======");

    let input = include_str!("./input.txt");

    let ts = std::time::Instant::now();
    let ans1 = input
        .lines()
        .map(parse_ranges)
        .filter(|(r1, r2)| (r1.0 >= r2.0 && r1.1 <= r2.1) || (r2.0 >= r1.0 && r2.1 <= r1.1))
        .count();
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = input
        .lines()
        .map(parse_ranges)
        .filter(|(r1, r2)| (r1.0 >= r2.0 && r1.0 <= r2.1) || (r2.0 >= r1.0 && r2.0 <= r1.1))
        .count();
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn parse_ranges(s: &str) -> ((i32, i32), (i32, i32)) {
    let (first, second) = s.split_once(',').unwrap();
    (parse_range(first), parse_range(second))
}

fn parse_range(s: &str) -> (i32, i32) {
    let (low, high) = s.split_once('-').unwrap();
    (low.parse::<i32>().unwrap(), high.parse::<i32>().unwrap())
}
