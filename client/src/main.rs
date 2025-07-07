use std::{
    io::{self, Error},
    process::exit,
};
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
    if let Err(e) = send_msg(&mut stream, "Hola Server".to_string()).await {
        eprintln!("Unable to send m: {e}");
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

    if let Err(e) = send_msg(&mut stream, nombre).await {
        eprintln!("Unable to send data: {e}");
    }

    if let Err(e) = send_msg(&mut stream, password).await {
        eprintln!("Unable to send data: {e}");
    }
}

async fn send_msg(stream: &mut TcpStream, msg: String) -> Result<String, Error> {
    let len = msg.len() as u8;
    stream.write_all(&[len]).await?;
    stream.write_all(msg.as_bytes()).await?;
    Ok("Msg sended succesfully".to_string())
}
