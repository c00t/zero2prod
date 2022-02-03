use zero2prod::run;

// use `cargo expand` to see expanded macros.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
