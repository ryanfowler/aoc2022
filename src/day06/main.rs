fn main() {
    println!("Day 06");
    println!("======");

    let input = include_str!("./input.txt").as_bytes();

    let ans1 = find_unique(input, 4);
    println!("Part 1: {}", ans1);

    let ans2 = find_unique(input, 14);
    println!("Part 2: {}", ans2);
}

fn find_unique(s: &[u8], n: usize) -> usize {
    for i in 0..s.len() - n {
        if is_unique(&s[i..i + n]) {
            return i + n;
        }
    }
    0
}

fn is_unique(s: &[u8]) -> bool {
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return false;
            }
        }
    }
    true
}
