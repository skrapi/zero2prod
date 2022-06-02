//! tests/health_check.rs
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // This spawns the app, and will drop and destroy it when the test completes
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Lauch app
fn spawn_app() -> String {
    // port 0 tells the OS to scan for an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address.");
    // spawn a new thread to run the server
    let _ = tokio::spawn(server);

    // Return address so that the caller can find the app
    format!("http://127.0.0.1:{}", port)
}
