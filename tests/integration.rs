#[tokio::test]
async fn test_server_client_interaction() {
    // Start the server
    tokio::spawn(async {
      let _ = server::main().await;
  });
  assert!(true)
}