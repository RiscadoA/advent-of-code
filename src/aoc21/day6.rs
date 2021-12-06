pub fn run(part: u32) {
    const MAX_DAY_COUNT: u64 = 256;

    let input: Vec<u64> = include_str!("../../input/2021/6.in")
        .split(",")
        .map(|s| {s.trim().parse().unwrap() })
        .collect();

    let mut dp: [u64; MAX_DAY_COUNT as usize + 7] = [0; MAX_DAY_COUNT as usize + 7];
    for n in 0..MAX_DAY_COUNT + 7 {
        dp[n as usize] = 1;
        for i in 0..(n / 7) {
            dp[n as usize] += dp[(n as i64 - 7 * i as i64 - 9).max(0) as usize];
        }
    }

    let day_count = if part == 1 { 80 } else { 256 }; 

    let mut ans = 0;
    for n in input {
        ans += dp[(day_count + 6 - n) as usize];
    }

    println!("{}", ans);
}
