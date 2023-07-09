use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:34197").await?;
    let sock2 = UdpSocket::bind("0.0.0.0:10000").await?;
    let remote_addr = "80.87.201.5:34197";
    sock2.connect(remote_addr);
    println!("Runned server!");
    let mut buf = [0; 1024];
    let mut client_addr = String::new();
    loop {
        println!("Waiting for input");
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}:", len, addr);
        if addr.to_string() != remote_addr {
            client_addr = addr.to_string();
            println!("Trying to send data to {}", remote_addr);
            let len = sock.send_to(&buf[..len], remote_addr).await?;
        } else {
            if client_addr.is_empty() {
                continue;
            }
            println!("Trying to send data to {}", &client_addr);
            let len = sock.send_to(&buf[..len], &client_addr).await?;
        }
    }
}
