extern crate openssl;

use openssl::rsa::Rsa;
use std::fs::File;
use std::io::Write;

/// [Generates public and private pem files]
pub fn generate_keys() {
    let rsa = Rsa::generate(2048).expect("Failed to generate");

    // Private key generation
    let private_key_pem = rsa.private_key_to_pem().expect("Failed to convert to pem");

    let mut private_key_file =
        File::create("private_key.pem").expect("Failed to create private key file");

    private_key_file
        .write_all(&private_key_pem)
        .expect("Failed to write private key pem");

    // Public key generation
    let public_key_pem = rsa.public_key_to_pem().expect("Failed to convert to pem");

    let mut public_key_file =
        File::create("public_key.pem").expect("Failed to create public key file");

    public_key_file
        .write_all(&public_key_pem)
        .expect("Failed to write public key pem");
}
