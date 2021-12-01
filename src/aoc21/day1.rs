pub fn run(part: u32) {
    let input: Vec<_> = include_str!("../../input/2021/1.in")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let answer = match part {
        1 => {
            input
                .iter()
                .fold((0, -1), |(p, c), &x| (x, if x > p { c + 1 } else { c }))
                .1
        }
        2 => {
            input
                .windows(3)
                .fold((0, -1), |(p, c), x| {
                    let sum = x.iter().sum();
                    (sum, if sum > p { c + 1 } else { c })
                })
                .1
        }
        _ => unreachable!(),
    };

    println!("{}", answer);
}
