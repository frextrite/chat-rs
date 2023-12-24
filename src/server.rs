use chat::{
    chatter_server::{Chatter, ChatterServer},
    ChatMessage, HealthReply, HealthRequest,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

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

    type SendAndReceiveMessagesStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn send_and_receive_messages(
        &self,
        _: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::SendAndReceiveMessagesStream>, Status> {
        unimplemented!()
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
