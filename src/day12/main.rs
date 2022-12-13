use std::collections::BinaryHeap;

fn main() {
    println!("Day 12");
    println!("======");

    let input = include_str!("./input.txt");
    let (map, start, end) = Map::parse(input);

    let ts = std::time::SystemTime::now();
    let ans1 = map.part1(start, end).unwrap();
    println!("Part 1: {} ({:?})", ans1, ts.elapsed().unwrap());

    let ts = std::time::SystemTime::now();
    let ans2 = map.part2(end).unwrap();
    println!("Part 2: {} ({:?})", ans2, ts.elapsed().unwrap());
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn parse(input: &str) -> (Self, (usize, usize), (usize, usize)) {
        let mut map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect())
            .collect();
        let start = find_byte(&map, b'S').unwrap();
        map[start.1][start.0] = b'a';
        let end = find_byte(&map, b'E').unwrap();
        map[end.1][end.0] = b'z';
        (Map(map), start, end)
    }

    fn part1(&self, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
        self.a_star(&[start], end)
    }

    fn part2(&self, end: (usize, usize)) -> Option<u32> {
        let mut starts = Vec::new();
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                if self.0[y][x] == b'a' {
                    starts.push((x, y));
                }
            }
        }
        self.a_star(&starts, end)
    }

    fn a_star(&self, starts: &[(usize, usize)], end: (usize, usize)) -> Option<u32> {
        let mut open = BinaryHeap::new();
        let mut seen = vec![vec![None; self.0[0].len()]; self.0.len()];
        for &start in starts {
            seen[start.1][start.0] = Some(0);
            open.push(Point {
                coords: start,
                distance: distance(start, end),
                steps: 0,
            });
        }

        while let Some(point) = open.pop() {
            if point.coords == end {
                return Some(point.steps);
            }
            if let Some(steps) = seen[point.coords.1][point.coords.0] {
                if point.steps > steps {
                    continue;
                }
            }

            let c = point.coords;
            for n in [
                (c.0 + 1, c.1),
                (c.0, c.1 + 1),
                (c.0 - 1, c.1),
                (c.0, c.1 - 1),
            ] {
                if n.0 >= self.0[0].len() || n.1 >= self.0.len() {
                    continue;
                }
                if self.0[n.1][n.0] > self.0[c.1][c.0] + 1 {
                    continue;
                }
                let new_steps = point.steps + 1;
                if let Some(v) = seen[n.1][n.0] {
                    if v <= new_steps {
                        continue;
                    }
                }
                seen[n.1][n.0] = Some(new_steps);
                open.push(Point {
                    coords: n,
                    distance: new_steps + distance(n, end),
                    steps: new_steps,
                });
            }
        }

        None
    }
}

fn distance(point: (usize, usize), end: (usize, usize)) -> u32 {
    (end.0.abs_diff(point.0) + end.1.abs_diff(point.1)) as u32
}

fn find_byte(map: &[Vec<u8>], b: u8) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find_map(|(x, &v)| if v == b { Some((x, y)) } else { None })
    })
}

struct Point {
    coords: (usize, usize),
    distance: u32,
    steps: u32,
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance) && self.steps.eq(&other.steps)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.distance.cmp(&self.distance) {
            std::cmp::Ordering::Equal => self.steps.cmp(&other.steps),
            o => o,
        }
    }
}
