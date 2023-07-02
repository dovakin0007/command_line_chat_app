use std::env;
use std::io::{Read, ErrorKind, Write};
use std::net::TcpListener;
use std::sync::mpsc;

const MSG_SIZE: usize = 32;

pub fn sleep(){
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let listener = TcpListener::bind("127.0.0.1:6000").expect("unable to bind");//creating our server
    listener.set_nonblocking(true).expect("failed to set nonblocking");

    let mut clients = vec![];// clients that are listening

    let (tx, rx) = mpsc::channel::<String>(); // creating channels

    loop {
        if let Ok((mut socket, addr)) = listener.accept() {// checking if the server returns socket and addr
            println!("Client connected successfully {:?}", addr);


            let tx = tx.clone();//cloning sender
            clients.push(socket.try_clone().expect("failed to connect to client")); //adding into clients

            std::thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); //collect all chars that are not white space
                    let msg = String::from_utf8(msg).expect("Invalid utf - 8 message"); //convert the characters into a string

                    println!("{} {:?}", addr, msg);

                    tx.send(msg).expect("Failed to send message to RX");
                    },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                
                Err(_) => {
                    println!("closing the connection");
                }
            }
            sleep();
            });

        }

        if let Ok(msg) = rx.try_recv(){
            clients= clients.into_iter().filter_map(|mut client| {
                // gets clients from client vec
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                //and gets the message sets it to client and writes it to all the clients in the client vector

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
            sleep();
        }
    }
}
