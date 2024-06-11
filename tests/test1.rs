#[cfg(test)]

mod tests {
    use dotenv::dotenv;

    #[test]
    fn test_case1() {
        dotenv().ok();

        assert!(true, "test case1 failed");
    }

    #[test]
    fn test_case2() {
        dotenv().ok();

        assert!(false, "test case2 failed");
    }
}
