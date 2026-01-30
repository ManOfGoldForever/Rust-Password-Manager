mod args;
mod storage;

use args::{Cli, Commands};
use clap::Parser;
use rpassword::read_password;
use std::io::Write;
use zeroize::Zeroize;

fn main() {
    let cli: Cli = Cli::parse();
    let master_hash_path = "master.hash";
    let file_path = "passwords.enc";
    let salt_path = "salt.bin";

    let encryption_salt = storage::get_or_create_salt(salt_path);

    let mut encryption_key = if !std::path::Path::new(master_hash_path).exists() {
        print!("No master password found. Setup required.\n");
        print!("Enter New Master Password: ");
        std::io::stdout().flush().expect("Flush failed");
        let mut masterp1 = read_password().expect("Failed to read password");

        print!("Confirm Master Password: ");
        std::io::stdout().flush().expect("Flush failed");
        let mut masterp2 = read_password().expect("Failed to read password");

        if masterp1 == masterp2 {
            println!("Creating new Master Password...");
            let hash = storage::hash_master_password(&masterp1);
            std::fs::write(master_hash_path, hash).expect("Failed to save hash");
            masterp2.zeroize();
            storage::derive_key(&mut masterp1, &encryption_salt)
        } else {
            masterp1.zeroize();
            masterp2.zeroize();
            panic!("Passwords did not match!");
        }
    } else {
        print!("Enter Master Password: ");
        std::io::stdout().flush().expect("Flush failed");
        let mut input = read_password().expect("Read failed");
        let saved_hash = std::fs::read_to_string(master_hash_path).expect("Failed to read hash");

        if !storage::verify_master_password(&input, &saved_hash) {
            input.zeroize();
            println!("Wrong Master Password! Access Denied.");
            return;
        }
        storage::derive_key(&mut input, &encryption_salt)
    };

    let mut passwords = storage::load_passwords(file_path, &encryption_key);

    match &cli.command {
        Commands::Add(args) => {
            print!("Enter password for {} : ", args.name);
            std::io::stdout().flush().expect("Flush failed");
            let mut p1 = read_password().expect("Failed to read password");

            print!("Confirm Password : ");
            std::io::stdout().flush().expect("Flush failed");
            let mut p2 = read_password().expect("Failed to read password");

            if p1 == p2 {
                passwords.insert(args.name.clone(), p1.clone());
                storage::save_passwords(file_path, &passwords, &encryption_key);
                println!("Saved successfully!");
            } else {
                println!("Passwords did not match!");
            }
            p1.zeroize();
            p2.zeroize();
        }
        Commands::Get(args) => match passwords.get(&args.name) {
            Some(pw) => println!("Password for {} : {}", args.name, pw),
            None => println!("No password found for '{}'", args.name),
        },
        Commands::Delete(args) => {
            storage::delete_password(file_path, &args.name, &encryption_key);
        }
        Commands::List => {
            if passwords.is_empty() {
                println!("No passwords saved yet!");
            } else {
                println!("--- Saved Passwords ---");
                for name in passwords.keys() {
                    println!("• {}", name);
                }
                println!("-----------------------");
            }
        }
    }

    encryption_key.zeroize();

    for (_, v) in passwords.iter_mut() {
        v.zeroize();
    }
}
