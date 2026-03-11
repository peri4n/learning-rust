use greeter::HelloRequest;
use greeter::greeter_client::GreeterClient;
use tonic::transport::Channel;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GreeterClient connecting to [::1]:50051");
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = GreeterClient::new(channel);
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
