use tokio::sync::oneshot;
use tokio::time::{self, Duration};

async fn some_computation() -> String {
    time::sleep(Duration::from_millis(100)).await;
    "after some computation".into()
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tokio::select! {
            val = some_computation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // `some_computation()` is canceled, the task completes and `tx1`
                // is dropped.
            }
        }
    });

    tokio::spawn(async move {
        time::sleep(Duration::from_millis(100)).await;
        let _ = tx2.send("two");
    });

    tokio::select! {
        Ok(val) = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        Ok(val) = rx2 => {
            println!("rx2 completed first with {:?}", val)
        }
    }
}
