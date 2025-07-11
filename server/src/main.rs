mod autenticable;
mod certificados;
mod credenciales;
mod gestion_cliente;
mod gestor_usuarios;

//FIXME quitar
mod debug;

use std::process::exit;
use std::sync::Arc;

use crate::certificados::get_certs;
use crate::debug::_ejecutar_sql;
use crate::gestion_cliente::{hashear_password, hilo_cliente};
use gestor_usuarios::GestorUsuarios;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let addr = "127.0.0.1:8080";
    let cert_addr = String::from("127.0.0.1");
    let cert_filename = String::from("cert.pem");
    let key_filename = String::from("key.pem");
    let db_path = "gestor.db";

    //Conectarse a la Base de Datos
    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Unable to established connection to DB: {e}");
            exit(1);
        }
    };

    _ejecutar_sql(&pool, "esquema.sql").await.unwrap();

    //Solucionar bootstrap creando usuario default
    asegurar_admin(&pool).await;

    //Cargar certificado y clave privada
    let (cert, key) = match get_certs(&cert_filename, &key_filename, &cert_addr) {
        Ok((c, k)) => (c, k),
        Err(e) => {
            eprintln!("Unable to generate cert: {e}");
            exit(1);
        }
    };

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert.clone()], key)
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));

    //Inicializar gestor de usuarios
    let gu = GestorUsuarios::new(pool);
    let gestor_usuarios = Arc::new(gu);

    //Inicializar conexion TCP
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Unable to connect to port 8080: {e}");
            return;
        }
    };

    println!("Server listening {}", addr);

    //Bucle lanzando un hilo cada vez que se conecta un cliente
    loop {
        match listener.accept().await {
            Ok((socket, socket_addr)) => {
                println!("Conectado cliente desde: {socket_addr}");
                let _ = tokio::spawn(hilo_cliente(
                    gestor_usuarios.clone(),
                    socket,
                    acceptor.clone(),
                ));
            }
            Err(e) => {
                eprintln!("Unable to connect to client: {e}");
            }
        }
    }
}

async fn asegurar_admin(pool: &Pool<Sqlite>) {
    //Comprobar que no exista ya un admin
    let count_admins: i64 =
        match sqlx::query_scalar("SELECT count(*) FROM usuario WHERE rol = 'admin'")
            .fetch_one(pool)
            .await
        {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Unable to count admins: {e}");
                return;
            }
        };

    if count_admins != 0 {
        return;
    }

    //Crear admin default
    let user =
        std::env::var("BOOTSTRAP_ADMIN_USER").expect("BOOTSTRAP_ADMIN_USER no está definido");
    let password =
        std::env::var("BOOTSTRAP_ADMIN_PASS").expect("BOOTSTRAP_ADMIN_PASS no está definido");

    let password = match hashear_password(&password) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Unable to hash default password: {e}");
            exit(1);
        }
    };

    if let Err(e) = sqlx::query!(
        "INSERT INTO usuario (nombre, password, rol)
             VALUES (?, ?, 'admin')",
        user,
        password
    )
    .execute(pool)
    .await
    {
        eprintln!("Unable to insert new admin: {e}");
        return;
    }

    //Borrar contraseña de entorno
    unsafe {
        std::env::remove_var("BOOTSTRAP_ADMIN_PASS");
    }

    println!("Default admin created: {}", user);
}
