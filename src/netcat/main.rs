use std::io::stdin;
use std::net::SocketAddr;

use anyhow::Result;

use command_line_rust::netcat::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};

async fn tcp_client(ip_addr: SocketAddr) -> Result<()> {
    let mut stream = TcpStream::connect(ip_addr).await?;
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        stream.write_all(buffer.as_bytes()).await?;
    }
}

async fn udp_client(ip_addr: SocketAddr) -> Result<()> {
    let stream = UdpSocket::bind(ip_addr).await?;
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        stream.send(buffer.as_bytes()).await?;
    }
}

async fn udp_server(ip_addr: SocketAddr) -> Result<()> {
    let socket = UdpSocket::bind(ip_addr).await?;
    let mut buf = [0; 1024];
    loop {
        let (n, _addr) = socket.recv_from(&mut buf).await?;
        let text = String::from_utf8_lossy(&buf[0..n]);
        print!("{}", text);
    }
}

async fn tcp_server(ip_addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(ip_addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let text = String::from_utf8_lossy(&buf[0..n]);

                print!("{}", text);
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let Config {
        ip_addr,
        listen,
        protocol,
    } = get_args();

    match (ip_addr, listen, protocol) {
        (ip_addr, true, Protocol::Tcp) => {
            tcp_server(ip_addr).await?;
        }
        (ip_addr, true, Protocol::Udp) => {
            udp_server(ip_addr).await?;
        }
        (ip_addr, false, Protocol::Tcp) => {
            tcp_client(ip_addr).await?;
        }
        (ip_addr, false, Protocol::Udp) => {
            udp_client(ip_addr).await?;
        }
    }

    Ok(())
}
