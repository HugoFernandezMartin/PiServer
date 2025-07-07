use std::collections::HashMap;

use tokio::sync::RwLock;

pub trait Autenticable {
    type Ok;
    type Err;

    async fn registrar_usuario(&mut self, nombre: String, password: String);
    async fn iniciar_sesion(
        &self,
        nombre: &String,
        password: &String,
    ) -> Result<Self::Ok, Self::Err>;
}
pub struct GestorUsuarios {
    usuarios: RwLock<HashMap<String, String>>,
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
    pub fn new() -> GestorUsuarios {
        //TODO cambiar por archivo
        Self {
            usuarios: RwLock::new(HashMap::new()),
        }
    }
}

impl Autenticable for GestorUsuarios {
    type Ok = String;
    type Err = String;
    async fn registrar_usuario(&mut self, nombre: String, password: String) {
        let mut mapa = self.usuarios.write().await;
        mapa.insert(nombre, password);
    }

    async fn iniciar_sesion(
        &self,
        nombre: &String,
        password: &String,
    ) -> Result<Self::Ok, Self::Err> {
        //Mirar si existe el usuario
        let mapa = self.usuarios.read().await;
        if mapa.contains_key(nombre) {
            if mapa.get(nombre).unwrap() == password {
                return Ok("Contraseña correcta".to_string());
            } else {
                Err("Contraseña incorrecta".to_string())
            }
        } else {
            Err("Usuario no existente".to_string())
        }
    }
}
