use std::io::Error;
use std::process::exit;
use std::sync::Arc;

use crate::autenticable::Autenticable;
use crate::credenciales::Credenciales;
use crate::gestor_usuarios::GestorUsuarios;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
pub async fn hilo_cliente(gestor_usuarios: Arc<GestorUsuarios>, mut socket: TcpStream) {
    //Obtener credenciales
    let credenciales: Credenciales = match recibir_credenciales(&mut socket).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error al recibir los credenciales: {e}");
            exit(1);
        }
    };
    println!(
        "Credenciales recibidos: \n    Nombre: {}\n    Password: {}",
        credenciales.get_nombre(),
        credenciales.get_password()
    );

    //Intentar iniciar sesion
    match gestor_usuarios.iniciar_sesion(credenciales).await {
        Ok(()) => {
            println!("Sesion iniciada correctamente");
        }
        Err(e) => {
            eprintln!("Error al iniciar sesión: {e}");
        }
    }
}

async fn recibir_credenciales(socket: &mut TcpStream) -> Result<Credenciales, Error> {
    let mut len_buf = [0u8; 1];
    socket.read_exact(&mut len_buf).await?;
    let len = len_buf[0] as usize;
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;

    let nombre = String::from_utf8_lossy(&buf).to_string();

    socket.read_exact(&mut len_buf).await?;
    let len = len_buf[0] as usize;
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;

    let password = String::from_utf8_lossy(&buf).to_string();

    Ok(Credenciales::new(
        nombre.trim().to_string(),
        password.trim().to_string(),
    ))
}

pub fn hashear_password(password: &String) -> Result<String, password_hash::Error> {
    //Generate salt
    let salt = SaltString::generate(&mut OsRng);

    //Crear Argon instance
    let argon2 = Argon2::default();

    // Hash password
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hashed_password)
}
