//! src/main.rs
use zero2prod::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our serve
    todo!()
    // let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    // run(listener)?.await
}