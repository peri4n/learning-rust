use std::io;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9090").await?;

    let mut buffer = [0_u8; 1024];

    loop {
        let (mut socket, _) = listener.accept().await?;
        socket.read(&mut buffer).await?;

        println!("Received data: {:?}", String::from_utf8_lossy(&buffer));
    }
}
