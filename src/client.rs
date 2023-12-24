use chat::{chatter_client::ChatterClient, HealthRequest};
use tonic::Request;

pub mod chat {
    tonic::include_proto!("chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatterClient::connect("http://[::1]:50051").await?;

    let request = Request::new(HealthRequest {});

    let response = client.probe_health(request).await?;

    println!("INFO: Received response {:?}", response);

    Ok(())
}
