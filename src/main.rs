use std::{net::UdpSocket, str};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use trust_dns_client::udp::UdpClientConnection;

struct Query {
    x: u8
}

fn main() -> std::io::Result<()> {
    {

        println!("startign the dns server");
        let socket = UdpSocket::bind("127.0.0.1:34001")?;
        let address = "8.8.8.8:53".parse().unwrap();
        let conn = UdpClientConnection::new(address).unwrap();
        println!("P{}", address);



        loop {

            println!("inside loop");
            let mut buffer = [0; 100];
            let (amt, src) = socket.recv_from(&mut buffer)?;
            let buffer = &mut buffer[..amt];
            let request = str::from_utf8(buffer).unwrap().trim();

            println!("recieved request {}", request);

            match request {
                "exit" => return Ok(()),
                _ => println!("requested resource is {}", request)
            }
            let query = Query{
                x: buffer[0]
            };
            println!("Query is {}", query.x);

            buffer.reverse();
            socket.send_to(buffer, src)?;
        }
    }
}
