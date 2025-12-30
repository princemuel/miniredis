use mini_redis::{Result, client};
use redis::address;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect(address(6379)).await?;

    client.set("hello", "redis".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={result:?}");

    Ok(())
}
