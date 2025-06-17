#[cfg(test)]
mod tests {
    use anyhow::Result;
    use dotenv::dotenv;

    #[test]
    fn test_case1() -> Result<()> {
        dotenv()?;

        assert!(true, "test case1 failed");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_case2() {
        dotenv().expect("env load failed!!!");

        panic!("test case2 failed");
    }
}
