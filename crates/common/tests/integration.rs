#[tokio::test]
async fn test_server_client_interaction() {
    // Arrange...
    let server_handle = tokio::spawn(async {
        let _ = server::run("127.0.0.1:8080".to_string()).await;
    });

    // Give the server some time to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client_handle = tokio::spawn(async {
        let _res = client::run("127.0.0.1:8080".to_string()).await;
    });
    let _ = tokio::join!(server_handle, client_handle);

    // Assert
}
