#[cfg(test)]
mod tests {
    use anyhow::Result;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_case3() -> Result<()> {
        dotenv()?;

        assert!(true, "test case3 failed");

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_case4() {
        dotenv().ok().expect("env load failed!!!");

        panic!("test case4 failed");
    }
}
