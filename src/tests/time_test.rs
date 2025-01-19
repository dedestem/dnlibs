#[cfg(test)]
mod time_tests {
    use crate::time;

    #[test]
    fn test_leapyear_true() {
        assert_eq!(true, time::is_leap_year(2024))
    }

    #[test]
    fn test_leapyear_false() {
        assert_eq!(false, time::is_leap_year(2025))
    }
}
