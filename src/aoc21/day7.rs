pub fn run(part: u32) {
    let mut input: Vec<i32> = include_str!("../../input/2021/7.in")
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let fuel = match part {
        1 => {
            // Median of the input
            input.sort();
            let position = input[input.len() / 2];

            // Get sum of distances to the position
            input.iter().map(|x| (x - position).abs()).sum::<i32>()
        }
        2 => {
            // Get the two possible averages (floored and ceiled)
            let sum = input.iter().sum::<i32>();
            let p1 = sum / input.len() as i32;
            let p2 = (sum + input.len() as i32 - 1) / input.len() as i32;

            // Get the fuel costs for each of the two possible averages
            let f1 = input.iter().map(|i| {
                let dist = (i - p1).abs() as i32;
                dist * (dist + 1) / 2
            }).sum::<i32>();

            let f2 = input.iter().map(|i| {
                let dist = (i - p2).abs() as i32;
                dist * (dist + 1) / 2
            }).sum::<i32>();

            // Return the smallest
            f1.min(f2)
        }
        _ => unreachable!(),
    };

    println!("{}", fuel);
}
