use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, ServerName, pem::PemObject},
};
use std::{
    io::{self, Error},
    process::exit,
    sync::Arc,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_rustls::{TlsConnector, client::TlsStream};
#[tokio::main]

async fn main() {
    let cert_filename = "cert.pem";
    let server_ip = "127.0.0.1";
    let server_name = match ServerName::try_from(server_ip) {
        Ok(sn) => sn,
        Err(e) => {
            eprintln!("Unable to parse serverIP: {e}");
            exit(1)
        }
    };

    //Cargar certificado
    let cert = match CertificateDer::from_pem_file(cert_filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Unable to open cert: {e}");
            exit(1)
        }
    };

    let mut root_store = RootCertStore::empty();
    if let Err(e) = root_store.add(cert) {
        eprintln!("Unable to add cert to rootStore: {e}");
    }

    // Crear configuración TLS del cliente
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    //Initialize TLS TCP connection
    let stream: TcpStream = match TcpStream::connect("127.0.0.1:8080").await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Unable to connect to server: {e}");
            exit(1);
        }
    };

    let mut tls_stream = match connector.connect(server_name, stream).await {
        Ok(tls) => tls,
        Err(e) => {
            eprintln!("Unable to encrypt TCP connection: {e}");
            exit(1)
        }
    };

    let (nombre, password) = obtener_credenciales().await;

    if let Err(e) = send_msg(&mut tls_stream, nombre).await {
        eprintln!("Unable to send data: {e}");
    }

    if let Err(e) = send_msg(&mut tls_stream, password).await {
        eprintln!("Unable to send data: {e}");
    }
    loop {}
}

async fn send_msg(stream: &mut TlsStream<TcpStream>, msg: String) -> Result<String, Error> {
    let len = msg.len() as u8;
    stream.write_all(&[len]).await?;
    stream.write_all(msg.as_bytes()).await?;
    Ok("Msg sended succesfully".to_string())
}

async fn obtener_credenciales() -> (String, String) {
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

    (nombre, password)
}
