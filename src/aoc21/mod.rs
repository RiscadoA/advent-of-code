mod day1;

pub fn run(day: u32, part: u32) -> bool {
    match day {
        1 => day1::run(part),
        _ => return false,
    }

    return true;
}
