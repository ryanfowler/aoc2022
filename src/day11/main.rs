fn main() {
    println!("Day 11");
    println!("======");

    let input = include_str!("./input.txt");
    let monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();
    for (index, monkey) in monkeys.iter().enumerate() {
        assert!(index == monkey.id);
    }

    let ts = std::time::Instant::now();
    let ans1 = monkey_business(monkeys.clone(), 20, Some(3));
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = monkey_business(monkeys, 10_000, None);
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

#[derive(Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    op: Operation,
    div: u64,
    ift: usize,
    iff: usize,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        Monkey {
            id: lines[0][7..lines[0].len() - 1].parse().unwrap(),
            items: lines[1][18..]
                .split(", ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect(),
            op: match lines[2][23..].split_once(' ').unwrap() {
                ("*", "old") => Operation::Square,
                ("*", n) => Operation::Mul(n.parse().unwrap()),
                ("+", n) => Operation::Add(n.parse().unwrap()),
                (op, _) => panic!("Unknown operator {}", op),
            },
            div: lines[3][21..].parse().unwrap(),
            ift: lines[4][29..].parse().unwrap(),
            iff: lines[5][30..].parse().unwrap(),
        }
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: u64, worry_relief: Option<u64>) -> u64 {
    let mut activity = vec![0; monkeys.len()];
    let modulo: u64 = monkeys.iter().map(|m| m.div).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            activity[i] += monkeys[i].items.len() as u64;
            while let Some(item) = monkeys[i].items.pop() {
                let item = match monkeys[i].op {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::Square => item * item,
                };
                let item = worry_relief.map_or_else(|| item % modulo, |v| item / v);
                let next = if item % monkeys[i].div == 0 {
                    monkeys[i].ift
                } else {
                    monkeys[i].iff
                };
                monkeys[next].items.push(item);
            }
        }
    }
    activity.sort_by(|a, b| b.cmp(a));
    activity.iter().take(2).product()
}
