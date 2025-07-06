use std::{io, process::exit};
use tokio::{io::AsyncWriteExt, net::TcpStream};
#[tokio::main]

async fn main() {
    //Initialize TCP connection
    let mut stream: TcpStream = match TcpStream::connect("127.0.0.1:8080").await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Unable to connect to server: {e}");
            exit(1);
        }
    };

    //Mandar handshake
    let msg = "Hola Server";
    let len = msg.len() as u8;
    if let Err(e) = stream.write_all(&[len]).await {
        eprintln!("Unable to send handshake: {e}");
    }
    if let Err(e) = stream.write_all(msg.as_bytes()).await {
        eprintln!("Unable to send msg: {e}");
    }

    //Obtener nombre
    println!("Nombre: ");
    let mut nombre = String::new();

    io::stdin()
        .read_line(&mut nombre)
        .expect("Failed to read line");

    //Obtener password
    println!("Password: ");
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let msg = "Hugo";
    let len = msg.len() as u8;
    if let Err(e) = stream.write_all(&[len]).await {
        eprintln!("Unable to send handshake: {e}");
    }
    if let Err(e) = stream.write_all(msg.as_bytes()).await {
        eprintln!("Unable to send msg: {e}");
    }

    let msg = "1234";
    let len = msg.len() as u8;
    if let Err(e) = stream.write_all(&[len]).await {
        eprintln!("Unable to send handshake: {e}");
    }
    if let Err(e) = stream.write_all(msg.as_bytes()).await {
        eprintln!("Unable to send msg: {e}");
    }
}
