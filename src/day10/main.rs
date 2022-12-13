fn main() {
    println!("Day 10");
    println!("======");

    let input = include_str!("./input.txt");

    let ts = std::time::SystemTime::now();
    let ans1 = signal_strengths(input);
    println!("Part 1: {} ({:?})", ans1, ts.elapsed().unwrap());

    let ts = std::time::SystemTime::now();
    let ans2 = render_crt(input);
    println!(
        "Part 2:\n{}\n({:?})",
        ans2.join("\n"),
        ts.elapsed().unwrap()
    );
}

fn signal_strengths(input: &str) -> i32 {
    let mut signal_strength = 0;
    let mut cycle = 0;
    let mut value = 1;
    for (cs, v) in input.lines().map(parse_instruction) {
        for _ in 0..cs {
            cycle += 1;
            if cycle % 40 == 20 {
                signal_strength += cycle * value;
            }
        }
        value += v;
    }
    signal_strength
}

fn render_crt(input: &str) -> Vec<String> {
    let mut value = 1;
    let mut screen = Vec::new();
    for (cs, v) in input.lines().map(parse_instruction) {
        for _ in 0..cs {
            if ((screen.len() % 40) as i32 - value).abs() <= 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }
        }
        value += v;
    }
    screen
        .chunks(40)
        .map(String::from_iter)
        .collect::<Vec<String>>()
}

fn parse_instruction(s: &str) -> (i32, i32) {
    if s == "noop" {
        (1, 0)
    } else {
        (2, s.split(' ').nth(1).unwrap().parse::<i32>().unwrap())
    }
}
