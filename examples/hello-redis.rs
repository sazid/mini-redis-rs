use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("Hello", "World".into()).await?;

    let result = client.get("Hello").await?;

    match &result {
        Some(bytes) => println!("{}", String::from_utf8_lossy(bytes)),
        None => println!("No results returned from the server"),
    }
    
    Ok(())
}
