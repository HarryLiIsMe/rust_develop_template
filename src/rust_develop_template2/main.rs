use anyhow::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let project_name = env::var("PROJECT_NAME").expect("env load failed!!!");

    println!("hello world2 {}", project_name);

    Ok(())
}
