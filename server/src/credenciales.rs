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
