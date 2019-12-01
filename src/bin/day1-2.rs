use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Part one

    let fuel: i64 = input.lines()
        .map(|i| i.parse().unwrap_or(0))
        .map(fuel_for_module)
        .map(fuel_for_fuel)
        .sum();

    println!("{}", fuel);

    Ok(())
}

fn fuel_for_module(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn fuel_for_fuel(fuel: i64) -> i64 {
    let mut total = fuel;
    let mut f = fuel;

    while f > 0 {
        f = fuel_for_module(f);
        total += f;
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_fuel() {
        assert_eq!(fuel_for_fuel(2), 2);
        assert_eq!(fuel_for_fuel(9), 10);
        assert_eq!(fuel_for_fuel(fuel_for_module(14)), 2);
        assert_eq!(fuel_for_fuel(fuel_for_module(101)), 39);
        assert_eq!(fuel_for_fuel(fuel_for_module(1969)), 966);
        assert_eq!(fuel_for_fuel(fuel_for_module(100756)), 50346);
    }
}