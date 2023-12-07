pub fn aoc2023_d01(input: String) -> u64 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let leading_digit = first_digit(l);
            let trailing_digit = last_digit(l);
            leading_digit * 10 + trailing_digit
        })
        .sum()
}

fn first_digit(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap()
        .into()
}

fn last_digit(input: &str) -> u64 {
    input
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn test_aoc2023_d01() {
        assert_eq!(aoc2023_d01(TEST_INPUT.to_string()), 142);
    }
}
