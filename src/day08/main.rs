fn main() {
    println!("Day 08");
    println!("======");

    let input = include_str!("./input.txt");
    let forest = Forest::parse(input);

    let ans1 = forest.visible_count();
    println!("Part 1: {}", ans1);

    let ans2 = forest.most_scenic();
    println!("Part 2: {}", ans2);
}

struct Forest(Vec<Vec<u32>>);

impl Forest {
    fn parse(s: &str) -> Self {
        let forest = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        Forest(forest)
    }

    fn visible_count(&self) -> usize {
        self.0.iter().enumerate().fold(0, |a, (row, v)| {
            a + v
                .iter()
                .enumerate()
                .filter(|&(col, _)| self.is_visible(row, col))
                .count()
        })
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let ht = self.0[row][col];
        self.0[row][..col].iter().all(|&v| v < ht)
            || self.0[row][col + 1..].iter().all(|&v| v < ht)
            || self.0[..row].iter().all(|v| v[col] < ht)
            || self.0[row + 1..].iter().all(|v| v[col] < ht)
    }

    fn most_scenic(&self) -> i32 {
        let mut best = 0;
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                best = best.max(self.scenic_score(row, col));
            }
        }
        best
    }

    fn scenic_score(&self, row: usize, col: usize) -> i32 {
        let height = self.0[row][col];
        let mut score = 1;
        score *= score_iter(height, self.0[row][..col].iter().rev().copied());
        score *= score_iter(height, self.0[row][col + 1..].iter().copied());
        score *= score_iter(height, self.0[..row].iter().map(|v| v[col]).rev());
        score *= score_iter(height, self.0[row + 1..].iter().map(|v| v[col]));
        score
    }
}

fn score_iter(height: u32, iter: impl Iterator<Item = u32>) -> i32 {
    let mut score = 0;
    for v in iter {
        score += 1;
        if v >= height {
            break;
        }
    }
    score
}
