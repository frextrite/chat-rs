use std::thread;

use chat::{chatter_client::ChatterClient, ChatMessage, HealthRequest};
use tokio::sync::mpsc;
use tonic::{transport::Endpoint, Request};

pub mod chat {
    tonic::include_proto!("chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Endpoint::from_static("http://[::1]:50051");
    let channel = endpoint.connect().await?;
    let mut client = ChatterClient::new(channel);

    let request = Request::new(HealthRequest {});
    let response = client.probe_health(request).await?;
    println!("INFO: Received response {:?}", response);

    let request = Request::new(HealthRequest {});
    let response = client.probe_health(request).await?;
    println!("INFO: Received response {:?}", response);

    let (tx, mut rx) = mpsc::channel(4);
    thread::spawn(move || {
        let input = std::io::stdin();
        let mut line = String::new();
        loop {
            input.read_line(&mut line).unwrap();
            let message = ChatMessage {
                author: "me".to_string(),
                message: line.clone(),
            };
            tx.blocking_send(message).unwrap();
            line.clear();
        }
    });
    let outbound = async_stream::stream! {
        while let Some(message) = rx.recv().await {
            yield message;
        }
    };
    let response = client
        .send_and_receive_messages(Request::new(outbound))
        .await?;
    let mut inbound = response.into_inner();
    while let Some(message) = inbound.message().await? {
        println!("INFO: Received message {:?}", message);
    }

    Ok(())
}
