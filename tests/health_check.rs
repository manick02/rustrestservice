use std::net::TcpListener;
use rustnewsletter::run;

#[tokio::test]
async fn health_check_works() {
// Arrange
    let app_url = spawn_app(); // We need to bring in `reqwest`
// to perform HTTP requests against our application. let client = reqwest::Client::new();
    // Act
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check",app_url))
        .send()
        .await
        .expect("Failed to execute request.");
// Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    // port o is a specialised port where OS scans list of available ports and uses it for binding
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}
