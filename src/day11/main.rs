fn main() {
    println!("Day 11");
    println!("======");

    let input = include_str!("./input.txt");
    let monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    let ans1 = monkey_business(monkeys.clone(), 20, true);
    println!("Part 1: {}", ans1);

    let ans2 = monkey_business(monkeys, 10_000, false);
    println!("Part 2: {}", ans2);
}

#[derive(Clone, Debug, Default)]
enum Operation {
    Add(u64),
    Mul(u64),
    #[default]
    Square,
}

#[derive(Clone, Debug, Default)]
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
        let mut monkey = Monkey::default();
        for (i, line) in input.lines().enumerate() {
            match i {
                0 => monkey.id = line[7..line.len() - 1].parse().unwrap(),
                1 => {
                    monkey.items = line[18..]
                        .split(", ")
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect()
                }
                2 => {
                    monkey.op = match line[23..].split_once(' ').unwrap() {
                        ("*", "old") => Operation::Square,
                        ("*", n) => Operation::Mul(n.parse().unwrap()),
                        ("+", n) => Operation::Add(n.parse().unwrap()),
                        (op, _) => panic!("Unknown operator {}", op),
                    }
                }
                3 => monkey.div = line[21..].parse().unwrap(),
                4 => monkey.ift = line[29..].parse().unwrap(),
                5 => monkey.iff = line[30..].parse().unwrap(),
                _ => {}
            }
        }
        monkey
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: u64, worry_relief: bool) -> u64 {
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
                let item = if worry_relief {
                    item / 3
                } else {
                    item % modulo
                };
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
