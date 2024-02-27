use anyhow::Result;
use my3::My3;
use tokio::signal;
#[tokio::main]
async fn main() -> Result<()> {
    let _my3 = My3::new();
    println!("Hello, world11!");
    signal::ctrl_c().await?;
    Ok(())
}
