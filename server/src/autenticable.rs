use crate::credenciales::Credenciales;

pub trait Autenticable {
    type Ok;
    type Err;

    async fn registrar_usuario(&self, nombre: String, password: String);
    async fn iniciar_sesion(&self, credenciales: Credenciales) -> Result<Self::Ok, Self::Err>;
}
