mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run(day: u32, part: u32) -> bool {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        4 => day4::run(part),
        5 => day5::run(part),
        6 => day6::run(part),
        _ => return false,
    }

    return true;
}
