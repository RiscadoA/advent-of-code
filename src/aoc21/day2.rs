pub fn run(part: u32) {
    let input = include_str!("../../input/2021/2.in").lines().map(|line| {
        let mut it = line.split(" ");
        let dir = it.next().unwrap();
        let dist = it.next().unwrap().parse::<i32>().unwrap();
        (dir, dist)
    });

    let position = match part {
        1 => input.fold((0, 0), |(x, y), (dir, dist)| match dir {
            "down" => (x, y + dist),
            "up" => (x, y - dist),
            "forward" => (x + dist, y),
            _ => panic!("Unknown direction: {}", dir),
        }),
        2 => {
            input
                .fold(((0, 0), 0), |((x, y), aim), (dir, dist)| match dir {
                    "down" => ((x, y), aim + dist),
                    "up" => ((x, y), aim - dist),
                    "forward" => ((x + dist, y + aim * dist), aim),
                    _ => panic!("Unknown direction: {}", dir),
                })
                .0
        }
        _ => unreachable!(),
    };

    println!("{}", position.0 * position.1);
}
