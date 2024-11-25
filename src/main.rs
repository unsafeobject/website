use warp::Filter;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let file = warp::path::end().map(|| "Hello this is a server");

    warp::serve(file).run(([192,168,42,213],8443)).await;
}
