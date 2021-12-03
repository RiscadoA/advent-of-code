pub fn run(part: u32) {
    let input = include_str!("../../input/2021/3.in")
        .lines()
        .map(|line| (line.len(), u32::from_str_radix(line, 2).unwrap()));
    // Make sure that length is the same for all lines.
    let len = input.clone().fold(0, |acc, (len, _)| {
        if acc == 0 {
            len
        } else {
            assert_eq!(acc, len);
            acc
        }
    });
    let input = input.map(|(_, v)| v);

    let answer = match part {
        1 => {
            let count = input.clone().count() as u32;
            let (gamma, epsilon) = (0..len)
                .map(|i| (i, input.clone().map(|v| (v >> i) & 1).sum::<u32>()))
                .map(|(i, c)| if c > count / 2 { (i, 1, 0) } else { (i, 0, 1) })
                .fold((0, 0), |(a, b), (i, c, d)| (a + (c << i), b + (d << i)));
            gamma * epsilon
        }
        2 => {
            let mut sorted: Vec<_> = input.collect();
            sorted.sort();
            let sorted = sorted;

            let mut o2 = (0, sorted.len() - 1);
            let mut co2 = (0, sorted.len() - 1);

            for i in (0..len).rev() {
                if o2.1 != o2.0 + 1 {
                    // Find the first index where the digit i is not 0.
                    let j = sorted[o2.0..o2.1].iter().position(|&v| (v >> i) & 1 != 0);
                    if let Some(j) = j {
                        if j > (o2.1 - o2.0) / 2 {
                            o2.1 = o2.0 + j;
                        } else {
                            o2.0 += j;
                        }
                    }
                }

                if co2.1 != co2.0 + 1 {
                    // Find the first index where the digit i is not 0.
                    let j = sorted[co2.0..co2.1].iter().position(|&v| (v >> i) & 1 != 0);
                    if let Some(j) = j {
                        if j > (co2.1 - co2.0) / 2 {
                            co2.0 += j;
                        } else {
                            co2.1 = co2.0 + j;
                        }
                    }
                }
            }

            assert_eq!(o2.0 + 1, o2.1);
            assert_eq!(co2.0 + 1, co2.1);
            sorted[o2.0] * sorted[co2.0]
        }
        _ => unreachable!(),
    };

    println!("{}", answer);
}
