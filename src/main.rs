use std::{net::UdpSocket, str};

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34001")?;

        loop {
            let mut buffer = [0; 10];
            let (amt, src) = socket.recv_from(&mut buffer)?;
            let request = str::from_utf8(&buffer).unwrap().to_string();

            match request.as_str() {
                "exit" => return Ok(()),
                _ => {
                    println!("requested {}", request);
                }
            }

            let buffer = &mut buffer[..amt];

            buffer.reverse();
            socket.send_to(buffer, &src)?;
        }
    }
}
