use aes::Aes256;
use ctr::Ctr64BE; 
use aes::cipher::{KeyIvInit, StreamCipher};  

type Aes256Ctr = Ctr64BE<Aes256>;
const KEY: &[u8] = b"725eff178731392b58e7e8b763cf2191";
const NONCE: &[u8] = b"316e6e95cab2701c";



//pega propriedades do sistema
pub fn get_prop(prop_name: &str) -> String {
    use std::process::Command;
    let output = Command::new("getprop")
        .arg(prop_name)
        .output()
        .expect("Failed to execute getprop command");

    let value = String::from_utf8_lossy(&output.stdout);
    value.trim().to_string()
}

pub fn decrypt(ciphertext: &[u8]) -> Vec<u8> {
    let mut cipher = Aes256Ctr::new(KEY.into(), NONCE.into());
    let mut buffer = ciphertext.to_vec();
    cipher.apply_keystream(&mut buffer);  
    buffer  // Retornar o texto decifrado
}
