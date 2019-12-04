use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let range: Vec<i32> = input.split('-')
        .map(|i| i.trim().parse().unwrap())
        .collect();

    let n = (range[0]..=range[1])
        .filter(|i| has_exactly_double_adjacent_digits(*i))
        .filter(|i| has_only_increasing_digits(*i))
        .count();

    println!("{}", n);

    Ok(())
}

fn has_exactly_double_adjacent_digits(n: i32) -> bool {
    let s = n.to_string();

    let mut digit_counter = 0;
    let mut last_digit = 'a';
    for ch  in s.chars() {
        if ch == last_digit {
            digit_counter += 1;
        } else if digit_counter == 2 {
            break
        } else {
            digit_counter = 1;
            last_digit = ch;
        }
    }
    match digit_counter {
        2 => true,
        _ => false,
    }
}

fn has_only_increasing_digits(n: i32) -> bool {
    let s = n.to_string();

    s.chars().zip(s.chars().skip(1))
        .all(|(a, b)| a <= b)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_digits() {
        assert_eq!(has_exactly_double_adjacent_digits(112233), true);
        assert_eq!(has_exactly_double_adjacent_digits(111122), true);

        assert_eq!(has_exactly_double_adjacent_digits(123444), false);
    }

    #[test]
    fn test_increasing_digits() {
        assert_eq!(has_only_increasing_digits(112345), true);
        assert_eq!(has_only_increasing_digits(111111), true);
        assert_eq!(has_only_increasing_digits(133579), true);

        assert_eq!(has_only_increasing_digits(123454), false);
        assert_eq!(has_only_increasing_digits(987654), false);
        assert_eq!(has_only_increasing_digits(333133), false);
    }
}