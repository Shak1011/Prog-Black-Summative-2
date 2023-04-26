use anyhow::{Result};
use sha2::{Sha256, Digest};
fn main() {
    let password = "yeet";
    if let Ok(hash) = Hash_file(password){
    dbg!(hash);
    println!("{}", hex_to_string(&hash));
    }
}

pub fn Hash_file(password: &str) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    hasher.update(&password);
    let hash= hasher.finalize();
    let mut ret: [u8; 32] = <[u8; 32]>::default();
    ret.copy_from_slice(&hash);
    return Ok(ret)
}

pub fn hex_to_string(data: &[u8]) -> String{
    let mut hashedpassword = String::new();
    for i in data {
        let x = format!("{:02x}", i);
        hashedpassword.push_str(&x);
    }
    hashedpassword
}
