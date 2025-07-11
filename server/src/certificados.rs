use std::{fs::write, path::Path};

use rcgen::{Certificate, KeyPair, generate_simple_self_signed};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};

pub fn get_certs<'a>(
    cert_filename: &String,
    key_filename: &String,
    cert_addr: &String,
) -> Result<(CertificateDer<'a>, PrivateKeyDer<'a>), String> {
    if !Path::new(&cert_filename).exists() || !Path::new(&key_filename).exists() {
        //Cargar certificado
        generate_new_certs(cert_filename, key_filename, cert_addr);
    }
    let cert = match CertificateDer::from_pem_file(cert_filename) {
        Ok(c) => c,
        Err(e) => return Err(format!("{}", e)),
    };
    let key = match PrivateKeyDer::from_pem_file(key_filename) {
        Ok(k) => k,
        Err(e) => return Err(format!("{}", e)),
    };

    Ok((cert, key))
}

fn generate_new_certs(
    cert_filename: &String,
    key_filename: &String,
    cert_addr: &String,
) -> (Certificate, KeyPair) {
    //Dominio que aceptará el certificado
    let subject_alt_names = vec![cert_addr.to_string()];

    //Generar certificado
    let certs = generate_simple_self_signed(subject_alt_names).unwrap();

    let cert = certs.cert;
    let key = certs.signing_key;
    //Guardar en disco
    write(cert_filename, cert.pem()).unwrap();
    write(key_filename, key.serialize_pem()).unwrap();

    (cert, key)
}
