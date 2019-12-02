use std::io::prelude::*;
use std::io::BufReader;

/// "to find the fuel required for a module, take its mass, divide by three, round down, and
/// subtract 2."
/// Therefore, we depend on integer division and then subtract 2
fn compute_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn compute_additional_fuel(initial_fuel: i32) -> i32 {
    let result = compute_fuel(initial_fuel);

    if result > 0 {
        result + compute_additional_fuel(result)
    } else {
        0
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let fuel = reader.lines().map(|line| {
        let fuel = compute_fuel(line.unwrap().parse().unwrap());
        fuel + compute_additional_fuel(fuel)
    }).sum::<i32>();
    println!("fuel: {}", fuel);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_provided_example_data() {
        assert_eq!(compute_fuel(12), 2);
        assert_eq!(compute_fuel(14), 2);
        assert_eq!(compute_fuel(1969), 654);
        assert_eq!(compute_fuel(100756), 33583);
    }

    #[test]
    fn it_handles_additional_fuel() {
        assert_eq!(compute_fuel(12) + compute_additional_fuel(compute_fuel(12)), 2);
        assert_eq!(compute_fuel(14) + compute_additional_fuel(compute_fuel(14)), 2);
        assert_eq!(compute_fuel(1969) + compute_additional_fuel(compute_fuel(1969)), 966);
        assert_eq!(compute_fuel(100756) + compute_additional_fuel(compute_fuel(100756)), 50346);
    }
}
