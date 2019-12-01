use std::io::prelude::*;
use std::io::BufReader;

/// "to find the fuel required for a module, take its mass, divide by three, round down, and
/// subtract 2."
/// Therefore, we depend on integer division and then subtract 2
fn compute_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    println!("{}", reader.lines().map(|line| compute_fuel(line.unwrap().parse().unwrap())).sum::<i32>());
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
}
