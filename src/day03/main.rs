fn main() {
    println!("Day 03");
    println!("======");

    let input = include_str!("./input.txt");

    let ts = std::time::SystemTime::now();
    let ans1: i32 = input.lines().fold(0, |total, line| {
        let (com1, com2) = line.split_at(line.len() / 2);
        let score = com1.chars().find_map(|c| {
            if com2.contains(c) {
                Some(get_score(c))
            } else {
                None
            }
        });
        total + score.unwrap()
    });
    println!("Part 1: {} ({:?})", ans1, ts.elapsed().unwrap());

    let ts = std::time::SystemTime::now();
    let mut ans2: i32 = 0;
    let mut lines = input.lines();
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        for c in l1.chars() {
            if l2.contains(c) && l3.contains(c) {
                ans2 += get_score(c);
                break;
            }
        }
    }
    println!("Part 2: {} ({:?})", ans2, ts.elapsed().unwrap());
}

fn get_score(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as i32
    } else {
        (c as u8 - b'A' + 27) as i32
    }
}
