use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Part one

    let fuel: i64 = input.lines()
        .map(|i| i.parse().unwrap_or(0))
        .map(fuel_for_module)
        .sum();

    println!("{}", fuel);

    Ok(())
}

fn fuel_for_module(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module(12), 2);
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 654);
        assert_eq!(fuel_for_module(100756), 33583);
        assert_eq!(fuel_for_module(6), 0);
        assert_eq!(fuel_for_module(1), 0);
    }
}