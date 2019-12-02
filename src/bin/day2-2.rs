use std::io::{self, Read};
use aoc_2019::computer::Computer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let values: Vec<i64> = input.split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect();

    for i in 0..99 {
        for j in 0..99 {
            let mut computer = Computer::new(values.clone());

            computer.set_noun(i)
                .set_verb(j)
                .execute();

            if computer.get_memory(0) == 19690720 {
                println!("{}", 100 * i + j);
            }
        }
    }

    Ok(())
}