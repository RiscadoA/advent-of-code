mod aoc21;

fn parse_problem_id() -> Option<(u32, u32, u32)> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return None;
    }

    let split = args[1].split('-').collect::<Vec<_>>();
    if split.len() != 3 {
        return None;
    }

    let year = split[0].parse::<u32>().ok()?;
    let day = split[1].parse::<u32>().ok()?;
    let part = split[2].parse::<u32>().ok()?;
    Some((year, day, part))
}

fn main() {
    // Parse the problem identifier.
    let (year, day, part) = match parse_problem_id() {
        Some(x) => x,
        None => {
            println!("Usage: {} <year>-<day>-<part>", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    // Check if the problem exists.
    if year != 2021 || part > 2 || !aoc21::run(day, part) {
        println!("Problem not found.");
        std::process::exit(1);
    }
}
