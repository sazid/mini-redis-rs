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

        tokio::spawn(async move {
            process(socket).await;
        });
    }

    // Ok(())
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();
    
    // The `Connection` lets us read/write redis **frames** instead of byte
    // streams. The `Connection` type is defined by min-redis.
    let mut connection = Connection::new(socket);

    // Use `read_frame()` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented command: {:?}", cmd),
        };
        
        // Write the response to the client.
        connection.write_frame(&response).await
            .expect("Failed to write frame to connection");
    }
}
