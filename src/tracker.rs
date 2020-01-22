use url::Url;

use std::net::ToSocketAddrs;
use std::net::UdpSocket;

extern crate rand;
use rand::Rng;


pub fn make_url(torrent_announce: String) -> String {
    let url = Url::parse(&torrent_announce[..]).expect("Couldn't parse url");
    let socket_url = format!("{}:{}", url.host().unwrap(), url.port().unwrap().to_string());
    socket_url
}

pub fn make_connect_request() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let buf: Vec<u8> = vec![0x00, 0x00, 0x04, 0x17, 0x27, 0x10, 0x19, 0x80, 0x00, 0x00, 0x00, 0x00, rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    //let connect_message: String = buf.into_iter().collect();
    //let mut buffer = ByteBuffer::new();
    //buffer.write_bytes(&vec![0x000041727101980, 0x00000000]);
    //buffer.write_u32(rng.gen::<u32>());
    buf
}

pub fn parse_connect_response(buf: [u8; 16]) {
    let action = &buf[0..4];
    let transaction_id = &buf[4..8];
    let connection_id = &buf[8..16];
    println!("action: {:?}, transaction_id: {:?}, connection_id: {:?}", action, transaction_id, connection_id);
}


pub fn udp_send(socket: UdpSocket, buf: Vec<u8>, torrent_announce: String) {
    let socket_url = make_url(torrent_announce);
    println!("socket_url: {:?}", socket_url);
    let mut addrs_iter = socket_url.to_socket_addrs().expect("Error transforming the address into a SocketAddr");
    let next_ip = addrs_iter.next().expect("couldn't get next ip in the ip iterator");
    println!("next_ip: {:?}", next_ip);
    socket.connect(next_ip).expect("Couldn't connect to the ip address with the socket"); 
    println!("{:?}", buf);
    let message = &buf[..];
    println!("{:?}", message);

    match socket.send(message) {
        Ok(number_of_bytes) => {
            println!("Number of bytes sent: {:?}", number_of_bytes);
            let mut buf = [0; 16];
            let (amt, src) = socket.recv_from(&mut buf).expect("Couldn't receive data from the target");
            parse_connect_response(buf);
            println!("Amount: {:?}, source: {:?}, buffer: {:?}", amt, src, buf);
        }
        Err(err) => println!("Error: {:?}", err), 
    }
}

