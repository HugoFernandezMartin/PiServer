use std::process::exit;
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

    if let Err(e) = stream.write("Hola Server".as_bytes()).await {
        eprintln!("Unable to send msg: {e}");
    }
}
