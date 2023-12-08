use std::str::FromStr;

pub fn aoc2023_d01_p1(input: String) -> u64 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let leading_digit = first_digit_p1(l);
            let trailing_digit = last_digit_p1(l);
            leading_digit * 10 + trailing_digit
        })
        .sum()
}

fn first_digit_p1(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap()
        .into()
}

fn last_digit_p1(input: &str) -> u64 {
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

#[derive(Debug)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl std::str::FromStr for Digit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zero" | "0" => Ok(Self::Zero),
            "one" | "1" => Ok(Self::One),
            "two" | "2" => Ok(Self::Two),
            "three" | "3" => Ok(Self::Three),
            "four" | "4" => Ok(Self::Four),
            "five" | "5" => Ok(Self::Five),
            "six" | "6" => Ok(Self::Six),
            "seven" | "7" => Ok(Self::Seven),
            "eight" | "8" => Ok(Self::Eight),
            "nine" | "9" => Ok(Self::Nine),
            _ => Err("Not a digit".to_string()),
        }
    }
}

impl std::convert::From<Digit> for u64 {
    fn from(value: Digit) -> Self {
        match value {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

#[derive(Debug)]
struct Digits<'a> {
    source: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Digits<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            end: 1,
        }
    }

    fn increment_counters(&mut self) {
        if self.is_done() {
            return;
        } else if self.end == self.start + 5 || self.end == self.source.len() {
            self.start += 1;
            self.end = self.start + 1;
        } else {
            self.end += 1;
        }
    }

    fn is_done(&self) -> bool {
        self.start == self.source.len() && self.end == self.source.len() + 1
    }

    fn get_substr(&self) -> &str {
        &self.source[self.start..self.end]
    }

    fn get_current_digit(&self) -> Option<Digit> {
        Digit::from_str(self.get_substr()).ok()
    }
}

impl<'a> Iterator for Digits<'a> {
    type Item = Digit;
    // not the most efficient (gettin current digit twice), result is still basically instant
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }

        let mut d = self.get_current_digit();
        while let None = d {
            self.increment_counters();
            if self.is_done() {
                return None;
            }
            d = self.get_current_digit();
        }

        self.increment_counters();
        d
    }
}

pub fn aoc2023_d01_p2(input: String) -> u64 {
    input
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            return Some(l.trim());
        })
        .map(get_number)
        .sum()
}

fn get_number(input: &str) -> u64 {
    let leading_digit = first_digit_p2(input);
    let trailing_digit = last_digit_p2(input);
    leading_digit * 10 + trailing_digit
}

fn first_digit_p2(input: &str) -> u64 {
    Digits::new(input).nth(0).unwrap().into()
}

fn last_digit_p2(input: &str) -> u64 {
    let digits = Digits::new(input).collect::<Vec<_>>();
    digits.into_iter().rev().nth(0).unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn test_aoc2023_d01_p1() {
        assert_eq!(aoc2023_d01_p1(TEST_INPUT1.to_string()), 142);
    }

    const TEST_INPUT2_1: &str = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    const TEST_INPUT2_2: &str = "
        eighthree
        sevenine";

    #[test]
    fn test_aoc2023_d01_p2() {
        assert_eq!(aoc2023_d01_p2(TEST_INPUT2_1.to_string()), 281);
        assert_eq!(aoc2023_d01_p2(TEST_INPUT2_2.to_string()), 162);
    }

    #[test]
    fn test_get_number() {
        macro_rules! assert_get_number {
            ($s:literal, $n:literal) => {
                assert_eq!(get_number($s), $n, $s)
            };
        }
        // part 1
        assert_get_number!("1abc2", 12);
        assert_get_number!("pqr3stu8vwx", 38);
        assert_get_number!("a1b2c3d4e5f", 15);
        assert_get_number!("treb7uchet", 77);

        // part 2
        assert_get_number!("two1nine", 29);
        assert_get_number!("eightwothree", 83);
        assert_get_number!("abcone2threexyz", 13);
        assert_get_number!("xtwone3four", 24);
        assert_get_number!("4nineeightseven2", 42);
        assert_get_number!("zoneight234", 14);
        assert_get_number!("7pqrstsixteen", 76);

        // overlaps
        assert_get_number!("eighthree", 83);
        assert_get_number!("sevenine", 79);

        // dafuq?
        assert_get_number!("4v35q", 45);
        assert_get_number!("65r", 65);
    }
}
