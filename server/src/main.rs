use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    //Inicializar conexion TCP
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Unable to connect to port 8080: {e}");
            return;
        }
    };

    match listener.accept().await {
        Ok((mut socket, _socket_addr)) => {
            let mut buf = [0; 1024];
            let n = match socket.read(&mut buf).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Unable to read: {e}");
                    return;
                }
            };
            let msg = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("{}", msg);
        }
        Err(e) => {
            eprintln!("Unable to connect to client: {e}");
        }
    }
}
