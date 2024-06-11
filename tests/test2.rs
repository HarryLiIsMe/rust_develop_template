#[cfg(test)]

mod tests {
    use dotenv::dotenv;

    #[test]
    fn test_case3() {
        dotenv().ok();

        assert!(true, "test case3 failed");
    }

    #[test]
    fn test_case4() {
        dotenv().ok();

        assert!(false, "test case3 failed");
    }
}
