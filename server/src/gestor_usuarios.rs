use sqlx::{Pool, Sqlite};

pub trait Autenticable {
    type Ok;
    type Err;

    async fn registrar_usuario(&self, nombre: String, password: String);
    async fn iniciar_sesion(
        &self,
        nombre: &String,
        password: &String,
    ) -> Result<Self::Ok, Self::Err>;
}
pub struct GestorUsuarios {
    pool: Pool<Sqlite>,
}

pub struct Credenciales {
    nombre: String,
    password: String,
}

impl Credenciales {
    pub fn new(nombre: String, password: String) -> Credenciales {
        Credenciales { nombre, password }
    }
    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }
    pub fn get_password(&self) -> &String {
        &self.password
    }
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
            "INSERT INTO usuarios (nombre, password) VALUES (?, ?)",
            nombre,
            password
        )
        .execute(&self.pool)
        .await
        {
            eprintln!("Unable to create user: {e}");
        } else {
            println!("User created succesfully");
        }
    }

    async fn iniciar_sesion(
        &self,
        nombre: &String,
        password: &String,
    ) -> Result<Self::Ok, Self::Err> {
        let usuario = sqlx::query!(
            "SELECT nombre, password FROM usuarios WHERE nombre = ?",
            nombre
        )
        .fetch_optional(&self.pool)
        .await;

        match usuario {
            Ok(Some(user)) => {
                if &user.password == password {
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
