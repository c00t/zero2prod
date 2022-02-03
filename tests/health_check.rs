//! health_check.rs
// `#[tokio::test]` is the test equivalent of `tokio::main`.
#[tokio::test]
async fn health_check_works() {
    // spawn our app
    spawn_app();
    // bring in reqwest to perform http requests
    let client = reqwest::Client::new();
    // send request to our server
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to send an request to `health_check`.");

    // check result
    // success?
    assert!(response.status().is_success());
    // body is empty?
    assert_eq!(response.content_length(), Some(0));
}
// we need to run this task as background task.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");
    let _ = tokio::spawn(server);
}
