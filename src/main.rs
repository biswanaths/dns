use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::{net::UdpSocket, str};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;


struct DnsHeaderFlags {
    pub qr      : bool,
    pub opcode  : u8,   // should this be enum? values are 0,1,2, two bits should be sufficient, The type can be QUERY (standard query, 0), IQUERY (inverse query, 1), or STATUS (server status request, 2).
    pub aa      : bool, // authorative answer
    pub tc      : bool, // truncation, if the message is truncated
    pub rd      : bool, // recursive needed for query
    pub ra      : bool, // recursion availabe, in a response
    pub z       : bool, // future use not used currently
    pub ad      : bool, // if the dns server verified data, in response
    pub cd      : bool, // verification disabled, in query, stating no verifiable data is acceptable in a response
    pub rcode   : u8,   // Response code, can be NOERROR (0), FORMERR (1, Format error), SERVFAIL (2), NXDOMAIN (3, Nonexistent domain)
}

struct DnsHeader {
    pub id      : u16,  // transaction id
    pub flags   : DnsHeaderFlags,
    pub qdcount : u16,  // Number of questions
    pub ancount : u16,  // Number of answers
    pub ns      : u16,  // Number of authority records
    pub arcount : u16,  // Number of additional records
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
        println!("listening inside loop");
        let mut buffer = [0; 100];
        let (amt, src) = socket.recv_from(&mut buffer)?;
        let buffer = &mut buffer[..amt];
        let request = str::from_utf8(buffer).unwrap().trim();

        println!("recieved request {}", request);
        println!("There is something in the rain.");

        match request {
            "exit" => return Ok(()),
            _ => println!("requested resource is {}", request),
        }

        let query = Name::from_str(request).unwrap();
        let response: DnsResponse = client.query(&query, DNSClass::IN, RecordType::A).unwrap();
        println!("response is {}", response.to_string());

        for answer in response.answers() {
            socket.send_to(answer.to_string().as_bytes(), src)?;
            socket.send_to("\n".as_bytes(), src)?;
        }
    }
}
