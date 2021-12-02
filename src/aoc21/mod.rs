mod day1;
mod day2;

pub fn run(day: u32, part: u32) -> bool {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        _ => return false,
    }

    return true;
}
