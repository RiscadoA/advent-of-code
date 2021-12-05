struct Board(Vec<i32>);

impl Board {
    fn new(numbers: Vec<i32>) -> Self {
        assert_eq!(numbers.len(), 5 * 5);
        Board(numbers)
    }

    fn from_str(input: &str) -> Self {
        let numbers = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        Self::new(numbers)
    }

    fn mark(&mut self, value: i32) -> bool {
        for r in 0..5 {
            for c in 0..5 {
                if self.0[r * 5 + c] == value {
                    self.0[r * 5 + c] = -1;

                    // Check if the entire column is marked
                    let mut won = true;
                    for k in 0..5 {
                        if self.0[k * 5 + c] != -1 {
                            won = false;
                        }
                    }

                    if won {
                        return true;
                    }

                    // Check if the entire row is marked
                    let mut won = true;
                    for k in 0..5 {
                        if self.0[r * 5 + k] != -1 {
                            won = false;
                        }
                    }

                    if won {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for r in 0..5 {
            for c in 0..5 {
                if self.0[r * 5 + c] != -1 {
                    score += self.0[r * 5 + c];
                }
            }
        }
        score
    }
}

pub fn run(part: u32) {
    let mut input = include_str!("../../input/2021/4.in").split("\n\n");
    let numbers: Vec<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    match part {
        1 => {
            let mut boards: Vec<_> = input.map(|x| Board::from_str(x)).collect();
            for number in numbers {
                for board in &mut boards {
                    if board.mark(number) {
                        println!("{}", board.score() * number);
                        return;
                    }
                }
            }
        }
        2 => {
            let mut won_count = 0;
            let mut boards: Vec<(bool, _)> = input.map(|x| (false, Board::from_str(x))).collect();
            let count = boards.len();
            for number in numbers {
                for board in &mut boards {
                    if !board.0 && board.1.mark(number) {
                        board.0 = true;
                        won_count += 1;
                        if won_count == count {
                            println!("{}", board.1.score() * number);
                            return;
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}
