use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    eprintln!("Usage: {} [send|recv] [address]", args[0]);
    return Ok(());
  }

  let mode = &args[1];
  let addr = &args[2];

  match mode.as_str() {
    "send" => send(addr).await?,
    "recv" => recv(addr).await?,
    _ => eprintln!("Invalid mode: {}", mode),
  }

  Ok(())
}

async fn send(addr: &str) -> Result<(), Box<dyn Error>> {
  let socket = UdpSocket::bind("0.0.0.0:0").await?;
  let to = addr.parse::<SocketAddr>()?;

  let message = b"Hello, UDP!";
  socket.send_to(message, &to).await?;

  println!("Sent message to {}: {}", to, String::from_utf8_lossy(message));

  Ok(())
}

async fn recv(addr: &str) -> Result<(), Box<dyn Error>> {
  let socket = UdpSocket::bind(addr).await?;
  let mut buf = vec![0; 1024];

  // let remote_addr = "127.0.0.1:5055";

  loop {
    let (len, addr) = socket.recv_from(&mut buf).await?;
    let message = &buf[..len];
    println!("{:?} bytes received from {:?}", len, addr);
    println!("Received message from {}: {}", addr, String::from_utf8_lossy(message));

    // send(remote_addr).await?;
  }
  // Ok(())
}