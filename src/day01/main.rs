fn main() {
    println!("Day 01");
    println!("======");

    let input = include_str!("./input.txt");
    let mut calories = input
        .split("\n\n")
        .map(|items| items.lines().map(|item| item.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    calories.sort_by(|a, b| b.cmp(a));

    let ans1: i32 = calories.iter().take(1).sum();
    println!("Part 1: {}", ans1);

    let ans2: i32 = calories.iter().take(3).sum();
    println!("Part 2: {}", ans2);
}
