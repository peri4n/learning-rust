use std::net::TcpStream;
use std::time::Duration;
use tokio::task;

async fn scan_port(port: u16, target: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", target, port);
    let saddr = addr.parse::<std::net::SocketAddr>()?;
    match TcpStream::connect_timeout(&saddr, Duration::from_millis(100)) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = "127.0.0.1"; // Replace with your target

    let mut handles = vec![];

    for port in 20..=100 {
        let handle = task::spawn(async move {
            scan_port(port, target).await.unwrap_or(false)
        });

        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await? {
            true => println!("Port {} is open", i + 20),
            false => println!("Port {} is closed", i + 20),
        }
    }

    Ok(())
}
