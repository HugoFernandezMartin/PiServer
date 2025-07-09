use sqlx::{Pool, Sqlite};

use crate::{autenticable::Autenticable, credenciales::Credenciales};

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
        if let Err(e) = sqlx::query!(
            "INSERT INTO usuario (nombre, password, rol) VALUES (?, ?, ?)",
            nombre,
            password,
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
                if &user.password == credenciales.get_password() {
                    return Ok(());
                } else {
                    return Err("Contraseña incorrecta".to_string());
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
