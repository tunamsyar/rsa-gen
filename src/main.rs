extern crate rsa_gen;

use rsa_gen::rsa::generate::generate_keys;
fn main() {
    generate_keys();
    println!("hello")
}
