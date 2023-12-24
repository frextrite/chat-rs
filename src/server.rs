use chat::{
    chatter_server::{Chatter, ChatterServer},
    HealthReply, HealthRequest,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod chat {
    tonic::include_proto!("chat");
}

#[derive(Debug, Default)]
pub struct SimpleChatter {}

#[tonic::async_trait]
impl Chatter for SimpleChatter {
    async fn probe_health(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthReply>, Status> {
        println!("INFO: Received request from {:?}", request.remote_addr());

        Ok(Response::new(HealthReply {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chatter = SimpleChatter::default();

    println!("INFO: Starting gRPC server on {:?}", addr);

    Server::builder()
        .add_service(ChatterServer::new(chatter))
        .serve(addr)
        .await?;

    Ok(())
}
