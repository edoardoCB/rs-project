use std::net::TcpListener;
use rs_project::run;

#[tokio::test]
async fn test_health() {
    let address = spawn_app().await;
    let url = format!("{address}/health");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).await.expect("Failed to listen port");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
