use async_std::task;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tide::{sse, Request};

#[async_std::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");

    struct State {
        counter: AtomicU64,
    };
    let state = State {
        counter: AtomicU64::new(1),
    };

    let mut app = tide::with_state(state);
    app.at("/")
        .get(sse::endpoint(|req: Request<State>, sender| async move {
            let n = req.state().counter.fetch_add(1, Ordering::SeqCst);
            let pod = PrintOnDrop { n };

            loop {
                println!("[{}] Sending...", pod.n);
                sender.send("hello", "world", None).await?;
                println!("[{}] Sent!", pod.n);
                task::sleep(Duration::from_secs(1)).await;
            }
        }));

    println!("Run this in another terminal:");
    println!("$ curl -v --no-buffer http://localhost:4141");
    println!("Wait a few seconds, then kill it");
    app.listen("localhost:4141").await.unwrap();
}

struct PrintOnDrop {
    n: u64,
}

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("[{}] Dropped!", self.n);
    }
}
