use url::Url;

use std::net::ToSocketAddrs;
use std::net::UdpSocket;

extern crate rand;
use rand::Rng;

use bytebuffer::ByteBuffer;

pub fn make_url(torrent_announce: String) -> String {
    let url = Url::parse(&torrent_announce[..]).expect("Couldn't parse url");
    let socket_url = format!("{}:{}", url.host().unwrap(), url.port().unwrap().to_string());
    socket_url
}

pub fn make_connect_request() -> String {
    let mut rng = rand::thread_rng();
    //let mut buf: Vec<String> = vec![String::new(); 4];
    //buf[0] = String::from("000417");
    //buf[1] = String::from("27101980");
    //buf[2] = String::from("00000000");
    //buf[3] = rng.gen::<u32>().to_string();
    //let connect_message: String = buf.into_iter().collect();
    let mut buffer = ByteBuffer::new();
    buffer.write_bytes(&vec![0x

    connect_message
}


pub fn udp_send(socket: UdpSocket, message: String, torrent_announce: String) {
    let socket_url = make_url(torrent_announce);
    println!("socket_url: {:?}", socket_url);
    let mut addrs_iter = socket_url.to_socket_addrs().expect("Error transforming the address into a SocketAddr");
    let next_ip = addrs_iter.next().expect("couldn't get next ip in the ip iterator");
    println!("next_ip: {:?}", next_ip);
    socket.connect(next_ip).expect("Couldn't connect to the ip address with the socket"); 
    println!("{:?}", message);
    let buf = message.as_bytes();
    println!("{:?}", buf);

    match socket.send(buf) {
        Ok(number_of_bytes) => {
            println!("Number of bytes sent: {:?}", number_of_bytes);
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf).expect("Couldn't receive data from the target");
            println!("Amount: {:?}, source: {:?}", amt, src);
        }
        Err(err) => println!("Error: {:?}", err), 
    }
}

