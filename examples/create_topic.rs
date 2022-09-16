use cloud_pubsub::Client;
use serde_derive::Deserialize;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let credentials_path = "/path/to/dummy_key.json".to_string();
    let pubsub = match Client::new(credentials_path).await {
        Err(e) => panic!("Failed to initialize pubsub: {}", e),
        Ok(p) => Arc::new(p),
    };

    let topic_name = "Test".to_string();
    let topic = Arc::new(pubsub.topic(topic_name));

    topic.create().await.expect("Topic Creation failed");

    match topic.clone().publish("🔥").await {
        Ok(response) => {
            println!("{:?}", response);
            pubsub.stop();
            std::process::exit(0);
        }
        Err(e) => eprintln!("Failed sending message {}", e),
    }
}
