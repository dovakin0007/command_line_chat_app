use std::io::{Read, Write, self};
use std::{net::TcpStream, sync::mpsc::channel};
use std::io::ErrorKind;
use std::thread;
fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6000").expect("unable to connect to server");
    client.set_nonblocking(true).expect("nonblocking initialization failed");

    let (tx, rx) = channel::<String>();

    thread::spawn(move || loop {
        let mut buf = vec![0; 32];
        match client.read_exact(&mut buf){
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message: {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Error has occurred connection severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(32, 0);
                client.write_all(&buff).expect("writing failed to socket");
                println!("message recieved: {}", msg);

            },
            Err(std::sync::mpsc::TryRecvError::Empty) => (),
            Err(std::sync::mpsc::TryRecvError::Disconnected) =>break,
        }
        thread::sleep(std::time::Duration::from_millis(100));
    });

    println!("Write a message :");
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("reading from stdin failed");
        let msg = buf.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}

    }
    println!("bye bye");
}
