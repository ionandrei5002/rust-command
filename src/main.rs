use std::io::{BufReader, BufWriter, Write, Read, self};
use std::os::unix::net::{UnixStream,UnixListener};

use serde::{Serialize, Deserialize};
use byteorder::{WriteBytesExt, BigEndian, ReadBytesExt};

#[derive(Serialize, Deserialize, Debug)]
enum MsgTypes {
    Register,
    Ok,
    Command,
    Close,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    header: MsgTypes,
    value: String,
}

fn read_message(stream: &UnixStream) -> String {
    let mut read = BufReader::new(stream);
    let mut msg_size = read.read_u64::<BigEndian>().unwrap() as usize;
    let mut line = String::from("");
    line.reserve(msg_size);

    while msg_size > 0 {
        let mut buffer = [0u8; 1024];
        let size = read.read(&mut buffer).unwrap();
        for ch in 0..size {
            line.push(char::from(buffer[ch]));
        }
        msg_size = msg_size - size;
    }
    println!("reading");
    return line;
}

fn write_message(stream: &UnixStream, message: &String) -> usize {
    let mut write = BufWriter::new(stream);
    let msg_size = message.as_bytes().len();
    if write.write_u64::<BigEndian>(msg_size as u64).is_err() {
        println!("Can't write to broker");
        return 0;
    }
    if write.write_all(message.as_bytes()).is_err() {
        println!("Can't write to broker");
        return 0;
    }
    if write.flush().is_err() {
        println!("Can't flush");
        return 0;
    }
    println!("writing");
    return msg_size;
}

fn main() {
    let path = String::from("/tmp/rust-uds.sock");

    let server = UnixStream::connect(path);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    println!("{}", buffer);

    match server {
        Ok(client) => {
            let msg = serde_json::to_string(&Message {header: MsgTypes::Ok, value: buffer.clone() }).unwrap();

            let size = write_message(&client, &msg);

            let line = read_message(&client);
            println!("{}", line);
        },
        _ => {}
    }
}
