use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process(socket).await?;
    }

    // Ok(())
}

async fn process(socket: TcpStream) -> Result<()> {
    // The `Connection` lets us read/write redis **frames** instead of byte
    // streams. The `Connection` type is defined by min-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await? {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await?;
    }
    
    Ok(())
}
