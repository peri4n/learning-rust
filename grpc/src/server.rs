use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status, transport::Server};

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[derive(Debug, Default)]
pub struct SimpleServer;

#[tonic::async_trait]
impl Greeter for SimpleServer {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = greeter::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GreeterServer listening on [::1]:50051");
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(GreeterServer::new(SimpleServer))
        .serve(addr)
        .await?;
    Ok(())
}
