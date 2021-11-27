use crossbeam::channel;
use futures::task::{self, ArcWake};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello World!");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task.
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a thread timer.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                // Notify the executor that this task is ready to be woken up.
                waker.wake();
            });

            Poll::Pending
        }
    }
}

// #[tokio::main]
// pub async fn main() {
//     let when = Instant::now() + Duration::from_millis(10);
//     let future = Delay { when };

//     let out = future.await;
//     assert_eq!(out, "done");
// }

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only one thread accesses
    // `future` at any given time.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // arc_self.schedule();
        arc_self.executor.send(arc_self.clone());
    }
}

impl Task {
    fn poll(self: Arc<Self>) {
        // Create a waker from the `Task` instance. This uses the `ArcWake` impl
        // from above.
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tries to lock the future.
        let mut future = self.future.try_lock().unwrap();

        // Poll the future.
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();

        Self { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // self.tasks.push_back(Box::pin(future));
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}
