


#[cfg(test)]
mod more_tests {
    use testing::*;

    #[test]
    pub fn test_big_boi() {
        let result = sploosh(splish(-1, 0), splish(1, 1), splish(3, 2));
        assert_eq!(result, 4);
    }
}