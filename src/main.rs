extern crate serde;
extern crate serde_bencode;

use serde::{Serialize, Deserialize};
use bencode::Bencode;
use serde_bytes::ByteBuf;


use std::io;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::net::UdpSocket;

#[derive(Debug, Deserialize)]
struct Node(String, i64);

//#[derive(Debug, Deserialize)]
//struct File {
//    path: Vec<String>,
//    length: i64,
//    md5sum: Option<String>,
//}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
struct Info {
    name: String,
    pieces: ByteBuf,
    pieces_length: i64,
    md5sum: Option<String>,
    length: Option<i64>,
    //files: Option<Vec<File>>,
    private: Option<bool>,
    path: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
struct Torrent {
    info: Info,
    node: Option<Vec<Node>>,
    annonce: Option<String>,
    annonce_list: Option<Vec<Vec<String>>>,
    creation_date: Option<i64>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
    httpseeds: Option<Vec<String>>,
}

impl Torrent {
    fn populate_from_bencode(&mut self, b: Bencode) -> Bencode {
        if let Bencode::Dict(dict) = b {
            for (s, b) in dict {
                if s.as_slice() == b"announce" {
                    self.annonce = extract_string(b);
                }
                else if s.as_slice() == b"created by" {
                    self.created_by = extract_string(b);
                }
                else if s.as_slice() == b"creation date" {
                    self.creation_date = extract_i64(b);
                }
                else if s.as_slice() == b"info" {
                    self.populate_from_bencode(b);
                }
            }
            return Bencode::Empty;
        }

        else {
            return b;
        }
    }
}

fn extract_string(b: Bencode) -> Option<String> {
    if let Bencode::ByteString(s) = b {
        return Some(String::from_utf8_lossy(&s).to_string());
    }
    return None;
}

fn extract_i64(b: Bencode) -> Option<i64> {
    if let Bencode::Number(s) = b {
        return Some(s);
    }
    return None;
}

fn create_torrent_from_file(path: &Path) -> Result<Torrent, Box<dyn std::error::Error>> {
    match std::fs::read(path) {
        Ok(v) => {
            let mut torrent: Torrent = serde_bencode::from_bytes(&v).unwrap();
            let bencode = bencode::from_buffer(&v).unwrap();
            torrent.populate_from_bencode(bencode);
            Ok(torrent)
        }
        Err(err) => panic!("Error: {}", err)
    }
}

fn create_socket(ip: String, message: String) -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.O.1:34254")?;
        socket.connect(ip)?;
        let buf = message.as_bytes();
        socket.send(buf)?;
        let mut buf = [0; 10]; 
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("amoun: {}, source: {}, buffer: {:?}", amt, src, buf);
    }
    Ok(())
}


fn main() {
    let torrent = create_torrent_from_file(Path::new("./puppy.torrent")).unwrap();
    println!("{:?}", torrent);
}
