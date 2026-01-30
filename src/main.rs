mod args;
mod storage;

use args::{Cli, Commands};
use clap::Parser;
use rpassword::read_password;

fn main() {
    let cli: Cli = Cli::parse();
    let master_hash_path = "master.hash";
    let file_path = "passwords.enc";
    let salt_path = "salt.bin";

    let encryption_salt = storage::get_or_create_salt(salt_path);

    let encryption_key: [u8; 32];

    if !std::path::Path::new(master_hash_path).exists() {
        print!("Enter Master Password : ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
        let masterp1 = read_password().expect("Failed to read password");

        print!("Confirm Master Password : ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
        let masterp2 = read_password().expect("Failed to read password");

        if masterp1 == masterp2 {
            println!("Creating new Master Password...");
            let hash = storage::hash_master_password(&masterp1);
            std::fs::write(master_hash_path, hash).expect("Failed to save hash");
            encryption_key = storage::derive_key(&masterp1, &encryption_salt);
        } else {
            println!("Passwords did not match! Please try again.");
            return;
        }
    } else {
        print!("Enter Master Password: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
        let input = read_password().expect("Read failed");
        let saved_hash = std::fs::read_to_string(master_hash_path).expect("Failed to read hash");
        if !storage::verify_master_password(&input, &saved_hash) {
            println!("Wrong Master Password! Access Denied.");
            return;
        }
        encryption_key = storage::derive_key(&input, &encryption_salt);
    }

    let mut passwords = storage::load_passwords(file_path, &encryption_key);

    match &cli.command {
        Commands::Add(args) => {
            print!("Enter password for {} : ", args.name);
            std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
            let p1 = read_password().expect("Failed to read password");
            print!("Confirm Password : ");
            std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
            let p2 = read_password().expect("Failed to read password");
            if p1 == p2 {
                passwords.insert(args.name.clone(), p1);
                storage::save_passwords(file_path, &passwords, &encryption_key);
                println!("Saved successfully!");
            } else {
                println!("Passwords did not match! Please try again.");
            }
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
        } // _ => {}
    }
}
