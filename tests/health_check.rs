//! health_check.rs
use std::net::TcpListener;

// `#[tokio::test]` is the test equivalent of `tokio::main`.
#[tokio::test]
async fn health_check_works() {
    // spawn our app
    let address = spawn_app();
    // bring in reqwest to perform http requests
    let client = reqwest::Client::new();
    // send request to our server
    let response = client
        .get(format!("http://{}/health_check",address)) // 使用&还是不用&都可以把，感觉没什么关系，毕竟后面也没用了
        .send()
        .await
        .expect("Failed to send an request to `health_check`.");

    // check result
    // success?
    assert!(response.status().is_success());
    // body is empty?
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_return_a_200_for_vaild_form_data(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions",address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(),200);
}

#[tokio::test]
async fn subscribe_return_a_400_for_missing_form_data(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let testcases = vec![
        ("", "missing both"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("name=le%20guin", "missing email")
    ];
    for (invalid_data, invalid_msg) in testcases{
        let response = client
            .post(format!("http://{}/subscriptions",address))
            .header("Content-Type","application/x-www-form-urlencoded")
            .body(invalid_data)
            .send()
            .await
            .expect("Failed to send request.");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            invalid_msg
        );
    }
}

// we need to run this task as background task.
fn spawn_app() -> String {
    // bind to port 0 will trigger a free ports scan
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}",port)
}
