use std::io::{self, Read};
use aoc_2019::computer::Computer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut values: Vec<i64> = input.split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect();

    values[1] = 12;
    values[2] = 2;

    let mut computer = Computer::new(values);
    computer.execute();

    println!("{}", computer.get_memory(0));

    Ok(())
}




