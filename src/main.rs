use std::net::TcpListener;
use zero2prod::run;

// use `cargo expand` to see expanded macros.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to 127.0.0.1:8080");
    run(listener)?.await
}
