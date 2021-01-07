use tikv_client::*;



#[tokio::main]
async fn main() {
    // Configure endpoints and optional TLS.
    let config = Config::default();

// Get a transactional client.
    let client = TransactionClient::new_with_config(
        vec![
            // A list of PD endpoints.
            // "192.168.0.100:2379",
            // "192.168.0.101:2379",
            "http://pd.tikv:2379",
        ], config).await.unwrap();

}




