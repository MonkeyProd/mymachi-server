use std::{io, str::from_utf8};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let client_sock = UdpSocket::bind("0.0.0.0:15151").await?;

    let sock = UdpSocket::bind("0.0.0.0:34197").await?;
    let remote_addr = "80.87.201.5:34197";
    println!("Runned server!");
    let mut buf = [0; 1024];
    let mut client_addr = String::new();
    loop {
        println!("Waiting for clients");
        let (len, addr) = client_sock.recv_from(&mut buf).await?;
        println!("Client connected");
        println!("Get this data from client: {}", from_utf8(&buf).unwrap());
        let len = sock
            .send_to(
                format!("Server is up {}", from_utf8(&buf[..len]).unwrap()).as_bytes(),
                addr,
            )
            .await?;

        // let (len, addr) = sock.recv_from(&mut buf).await?;
        // println!("{:?} bytes received from {:?}:", len, addr);
        // if addr.to_string() != remote_addr {
        //     client_addr = addr.to_string();
        //     println!("Trying to send data to {}", remote_addr);
        //     let len = sock.send_to(&buf[..len], remote_addr).await?;
        // } else {
        //     if client_addr.is_empty() {
        //         continue;
        //     }
        //     println!("Trying to send data to {}", &client_addr);
        //     let len = sock.send_to(&buf[..len], &client_addr).await?;
        // }
    }
}
