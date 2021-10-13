use std::sync::Arc;

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use mini_redis_rs::shard_db::ShardDb;
use tokio::net::{TcpListener, TcpStream};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;

type Db = Arc<ShardDb<String, Bytes>>;

#[tokio::main]
pub async fn main() -> Result<()> {
    let addr = format!("127.0.0.1:{}", mini_redis_rs::DEFAULT_PORT);

    // Bind the listener to the address
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    let shard_db = Arc::new(ShardDb::new(8));

    loop {
        let (socket, _) = listener.accept().await?;

        let shard_db = shard_db.clone();

        tokio::spawn(async move {
            process(socket, shard_db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // The `Connection` lets us read/write redis **frames** instead of byte
    // streams. The `Connection` type is defined by min-redis.
    let mut connection = Connection::new(socket);

    // Use `read_frame()` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut shard = db.get(cmd.key().to_string()).lock().unwrap();

                shard.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let shard = db.get(cmd.key().to_string()).lock().unwrap();

                if let Some(value) = shard.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented command: {:?}", cmd),
        };

        // Write the response to the client.
        connection
            .write_frame(&response)
            .await
            .expect("Failed to write frame to connection");
    }
}
