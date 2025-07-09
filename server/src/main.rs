mod autenticable;
mod credenciales;
mod gestion_cliente;
mod gestor_usuarios;

//FIXME quitar
mod debug;

use std::io::Error;
use std::process::exit;
use std::sync::Arc;

use crate::gestion_cliente::hilo_cliente;
use gestor_usuarios::GestorUsuarios;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let addr = "127.0.0.1:8080";
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

    //_ejecutar_sql(&pool, "esquema.sql").await.unwrap();

    //Solucionar bootstrap creando usuario default
    asegurar_admin(&pool).await;

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
            Ok((mut socket, socket_addr)) => {
                //Mandar handshake
                match handshake(&mut socket).await {
                    Ok(msg) => println!("{}: {}", socket_addr, msg),
                    Err(e) => eprintln!("Unable to established connection: {e}"),
                }
                let _ = tokio::spawn(hilo_cliente(gestor_usuarios.clone(), socket));
            }
            Err(e) => {
                eprintln!("Unable to connect to client: {e}");
            }
        }
    }
}

async fn handshake(socket: &mut TcpStream) -> Result<String, Error> {
    let mut len_buf = [0u8; 1];
    socket.read_exact(&mut len_buf).await?;
    let len = len_buf[0] as usize;
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
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
