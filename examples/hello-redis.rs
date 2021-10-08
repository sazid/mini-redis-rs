use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client1 = client::connect("127.0.0.1:6379").await?;
    let mut client2 = client::connect("127.0.0.1:6379").await?;

    // Set a value with client 1
    client1.set("Hello", "World".into()).await?;

    // Get the value with client 2
    let result = client2.get("Hello").await?;

    match &result {
        Some(bytes) => println!("{}", String::from_utf8_lossy(bytes)),
        None => println!("No results returned from the server"),
    }

    Ok(())
}
