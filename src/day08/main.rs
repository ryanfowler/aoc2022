fn main() {
    println!("Day 08");
    println!("======");

    let input = include_str!("./input.txt");
    let forest = Forest::parse(input);

    let ts = std::time::SystemTime::now();
    let ans1 = forest.total_visible();
    println!("Part 1: {} ({:?})", ans1, ts.elapsed().unwrap());

    let ts = std::time::SystemTime::now();
    let ans2 = forest.highest_scenic_score();
    println!("Part 2: {} ({:?})", ans2, ts.elapsed().unwrap());
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

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &u32)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(r, v)| v.iter().enumerate().map(move |(c, v)| (r, c, v)))
    }

    fn total_visible(&self) -> usize {
        self.iter()
            .filter(|&(row, col, _)| self.is_visible(row, col))
            .count()
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let height = self.0[row][col];
        self.0[row][..col].iter().all(|&v| v < height)
            || self.0[row][col + 1..].iter().all(|&v| v < height)
            || self.0[..row].iter().all(|v| v[col] < height)
            || self.0[row + 1..].iter().all(|v| v[col] < height)
    }

    fn highest_scenic_score(&self) -> i32 {
        self.iter()
            .map(|(row, col, _)| self.scenic_score(row, col))
            .fold(0, |best, v| best.max(v))
    }

    fn scenic_score(&self, row: usize, col: usize) -> i32 {
        let height = self.0[row][col];
        score_iter(height, self.0[row][..col].iter().rev().copied())
            * score_iter(height, self.0[row][col + 1..].iter().copied())
            * score_iter(height, self.0[..row].iter().map(|v| v[col]).rev())
            * score_iter(height, self.0[row + 1..].iter().map(|v| v[col]))
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
