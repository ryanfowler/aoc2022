use std::cmp::Ordering;

fn main() {
    println!("Day 13");
    println!("======");

    let input = include_str!("./input.txt").trim_end();
    let pairs: Vec<(Value, Value)> = parse_input(input);

    let ts = std::time::Instant::now();
    let ans1 = part1(&pairs);
    println!("Part 1: {} ({:?})", ans1, ts.elapsed());

    let ts = std::time::Instant::now();
    let ans2 = part2(&pairs);
    println!("Part 2: {} ({:?})", ans2, ts.elapsed());
}

fn part1(pairs: &[(Value, Value)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.0 < pair.1)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(pairs: &[(Value, Value)]) -> usize {
    let mut packets = pairs.iter().fold(Vec::new(), |mut acc, pair| {
        acc.push(pair.0.clone());
        acc.push(pair.1.clone());
        acc
    });
    let dec1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let dec2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);
    packets.push(dec1.clone());
    packets.push(dec2.clone());
    packets.sort();
    let idx1 = packets.iter().position(|v| v == &dec1).unwrap() + 1;
    let idx2 = packets.iter().position(|v| v == &dec2).unwrap() + 1;
    idx1 * idx2
}

#[derive(Clone)]
enum Value {
    List(Vec<Value>),
    Int(i32),
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(ln), Value::Int(rn)) => ln.cmp(rn),
            (Value::List(_), Value::Int(_)) => self.cmp(&Value::List(vec![other.clone()])),
            (Value::Int(_), Value::List(_)) => Value::List(vec![self.clone()]).cmp(other),
            (Value::List(ll), Value::List(rl)) => {
                for (v1, v2) in ll.iter().zip(rl) {
                    let res = v1.cmp(v2);
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                ll.len().cmp(&rl.len())
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            (parse_line(left.as_bytes()), parse_line(right.as_bytes()))
        })
        .collect()
}

fn parse_line(input: &[u8]) -> Value {
    let (val, n) = parse_list(input);
    assert!(n == input.len());
    val
}

fn parse_list(input: &[u8]) -> (Value, usize) {
    assert!(input[0] == b'[');
    let mut list = Vec::new();
    let mut i = 1;
    while i < input.len() {
        let (val, n) = match input[i] {
            b'[' => parse_list(&input[i..]),
            b']' => return (Value::List(list), i + 1),
            b',' => {
                i += 1;
                continue;
            }
            _ => parse_int(&input[i..]),
        };
        list.push(val);
        i += n;
    }
    (Value::List(list), input.len())
}

fn parse_int(input: &[u8]) -> (Value, usize) {
    let mut n = 0;
    let mut num: i32 = 0;
    for v in input {
        match v {
            b'[' | b']' | b',' => break,
            v => num = num * 10 + (v - b'0') as i32,
        }
        n += 1;
    }
    (Value::Int(num), n)
}
