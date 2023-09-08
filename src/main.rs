use config::Config;
use std::error::Error;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let f = std::fs::File::open("examples/sample.yaml")?;
    let mut c: Config = serde_yaml::from_reader(f)?;

    let _ = c.push_tasks().await?;

    Ok(())
}