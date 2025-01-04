use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let project_name = env::var("PROJECT_NAME").expect("env load failed!!!");

    println!("hello world2 {:?}", project_name);
}
