fn main() {
    println!("Day 02");
    println!("======");

    let input = include_str!("./input.txt");
    let ans1 = input.lines().fold(0, |score, line| {
        let them = Shape::parse(line.chars().next().unwrap()).unwrap();
        let me = Shape::parse(line.chars().nth(2).unwrap()).unwrap();
        let result = me.play(them);
        score + result.score() + me.score()
    });
    println!("Part 1: {}", ans1);

    let ans2 = input.lines().fold(0, |score, line| {
        let them = Shape::parse(line.chars().next().unwrap()).unwrap();
        let result = Result::parse(line.chars().nth(2).unwrap()).unwrap();
        let me = if Shape::Rock.play(them) == result {
            Shape::Rock
        } else if Shape::Paper.play(them) == result {
            Shape::Paper
        } else if Shape::Scissors.play(them) == result {
            Shape::Scissors
        } else {
            panic!("Could not find a solution");
        };
        score + result.score() + me.score()
    });
    println!("Part 2: {}", ans2);
}

#[derive(PartialEq)]
enum Result {
    Win,
    Draw,
    Loss,
}

impl Result {
    fn parse(c: char) -> Option<Result> {
        match c {
            'X' => Some(Self::Loss),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(c: char) -> Option<Shape> {
        match c {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, opponent: Shape) -> Result {
        match self {
            Self::Rock => match opponent {
                Self::Rock => Result::Draw,
                Self::Paper => Result::Loss,
                Self::Scissors => Result::Win,
            },
            Self::Paper => match opponent {
                Self::Rock => Result::Win,
                Self::Paper => Result::Draw,
                Self::Scissors => Result::Loss,
            },
            Self::Scissors => match opponent {
                Self::Rock => Result::Loss,
                Self::Paper => Result::Win,
                Self::Scissors => Result::Draw,
            },
        }
    }
}
