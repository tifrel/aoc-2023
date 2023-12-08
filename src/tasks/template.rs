pub fn aoc2023_dXX(input: String) -> String {
    input
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
    fn test_aoc2023_dXX() {
        assert_eq!(aoc2023_dXX(TEST_INPUT.to_string()), TEST_INPUT.to_string());
    }
}
