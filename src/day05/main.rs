fn main() {
    println!("Day 05");
    println!("======");

    let input = include_str!("./input.txt");
    let (raw_stacks, raw_instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Stacks::parse(raw_stacks);
    for inst in raw_instructions.lines().map(Instruction::parse) {
        stacks.move_crates_single(inst)
    }
    let ans1 = stacks.top_crates();
    println!("Part 1: {}", ans1);

    let mut stacks = Stacks::parse(raw_stacks);
    for inst in raw_instructions.lines().map(Instruction::parse) {
        stacks.move_crates_multi(inst)
    }
    let ans2 = stacks.top_crates();
    println!("Part 2: {}", ans2);
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(' ');
        Instruction {
            quantity: parts.nth(1).unwrap().parse::<usize>().unwrap(),
            from: parts.nth(1).unwrap().parse::<usize>().unwrap(),
            to: parts.nth(1).unwrap().parse::<usize>().unwrap(),
        }
    }
}
#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn parse(input: &str) -> Self {
        let mut stacks = input.lines().fold(Vec::new(), |mut stacks, line| {
            line.chars()
                .enumerate()
                .filter(|(n, c)| n % 4 == 1 && *c != ' ')
                .for_each(|(n, c)| {
                    let index = n / 4;
                    match stacks.get_mut(index) {
                        Some(stack) => stack,
                        None => {
                            if stacks.len() < index {
                                stacks.resize(index + 1, Vec::new())
                            }
                            &mut stacks[index]
                        }
                    }
                    .push(c);
                });
            stacks
        });
        for stack in stacks.iter_mut() {
            stack.pop();
            stack.reverse();
        }
        Stacks(stacks)
    }

    fn move_crates_multi(&mut self, inst: Instruction) {
        let from_stack = &mut self.0[inst.from - 1];
        let mut vals = from_stack.split_off(from_stack.len() - inst.quantity);
        self.0[inst.to - 1].append(&mut vals);
    }

    fn move_crates_single(&mut self, inst: Instruction) {
        for _ in 0..inst.quantity {
            self.move_crate(inst.from, inst.to);
        }
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let ch = self.0[from - 1].pop().unwrap();
        self.0[to - 1].push(ch);
    }

    fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}
