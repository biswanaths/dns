use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::{net::UdpSocket, str};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

struct Query {
    x: u8,
}

fn main() -> std::io::Result<()> {
    println!("startign the dns server");
    let socket = UdpSocket::bind("127.0.0.1:34001")?;
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    println!("P{}", address);
    let name = Name::from_str("www.google.com.").unwrap();
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    println!("Answers {}", response.answers()[0]);

    loop {
        println!("inside loop");
        let mut buffer = [0; 100];
        let (amt, src) = socket.recv_from(&mut buffer)?;
        let buffer = &mut buffer[..amt];
        let request = str::from_utf8(buffer).unwrap().trim();

        println!("recieved request {}", request);

        match request {
            "exit" => return Ok(()),
            _ => println!("requested resource is {}", request),
        }

        let query = Name::from_str(request).unwrap();
        let response: DnsResponse = client.query(&query, DNSClass::IN, RecordType::A).unwrap();

        for answer in response.answers() {
            socket.send_to(answer.to_string().as_bytes(), src)?;
            socket.send_to("\n".as_bytes(), src)?;
        }
    }
}
