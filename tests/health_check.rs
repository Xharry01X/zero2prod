#[tokio::test]
async fn health_check_works() {
    // Spawn the application
    spawn_app();

    // Create a client
    let client = reqwest::Client::new();
    
    // Send a request to the health check endpoint
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Helper function to start the application
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    
    // Spawn the server as a background task
    let _ = tokio::spawn(server);
    
    // Give the server a moment to start up
    std::thread::sleep(std::time::Duration::from_millis(100));
}