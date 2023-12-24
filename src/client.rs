use chat::{chatter_client::ChatterClient, HealthRequest};
use tonic::{
    transport::Endpoint,
    Request,
};

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

    Ok(())
}
