use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let project_name = env::var("PROJECT_NAME").unwrap();

    println!("hello world1 {}", project_name);
}
