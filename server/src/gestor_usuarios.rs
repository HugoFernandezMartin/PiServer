use std::process::exit;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::{Pool, Sqlite};

use crate::{
    autenticable::Autenticable, credenciales::Credenciales, gestion_cliente::hashear_password,
};

pub struct GestorUsuarios {
    pool: Pool<Sqlite>,
}

impl GestorUsuarios {
    pub fn new(pool: Pool<Sqlite>) -> GestorUsuarios {
        Self { pool }
    }
}

impl Autenticable for GestorUsuarios {
    type Ok = ();
    type Err = String;
    async fn registrar_usuario(&self, nombre: String, password: String) {
        let hashed_password = match hashear_password(&password) {
            Ok(hashed_password) => hashed_password,
            Err(e) => {
                eprintln!("Unable to hash password: {e}");
                exit(1);
            }
        };
        if let Err(e) = sqlx::query!(
            "INSERT INTO usuario (nombre, password, rol) VALUES (?, ?, ?)",
            nombre,
            hashed_password,
            "user"
        )
        .execute(&self.pool)
        .await
        {
            eprintln!("Unable to create user: {e}");
        } else {
            println!("User created succesfully");
        }
    }

    async fn iniciar_sesion(&self, credenciales: Credenciales) -> Result<Self::Ok, Self::Err> {
        let nombre = credenciales.get_nombre();
        let usuario = sqlx::query!(
            "SELECT nombre, password FROM usuario WHERE nombre = ?",
            nombre
        )
        .fetch_optional(&self.pool)
        .await;

        match usuario {
            Ok(Some(user)) => {
                let parsed_hash = match PasswordHash::new(&user.password) {
                    Ok(ph) => ph,
                    Err(e) => return Err(format!("Unable to create passwordHash object: {}", e)),
                };

                if let Err(e) = Argon2::default()
                    .verify_password(credenciales.get_password().as_bytes(), &parsed_hash)
                {
                    return Err(format!("{}", e));
                } else {
                    Ok(())
                }
            }
            Ok(None) => return Err("Usuario no existente".to_string()),
            Err(e) => {
                let e_msg = format!("{}", e);
                return Err(e_msg);
            }
        }
    }
}
