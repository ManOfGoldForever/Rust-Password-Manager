mod args;
mod storage;

use args::{Cli, Commands};
use clap::Parser;
use rpassword::read_password;
use std::io::Write;
use std::path::PathBuf;
use zeroize::Zeroize;

fn main() {
    let cli: Cli = Cli::parse();

    if cli.command.is_none() {
        print_banner();
        println!("Welcome to pm - The Rust Password Manager");
        println!("Use 'pm --help' to see available commands.");
        return;
    }

    let home_str = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory");

    let mut config_dir = PathBuf::from(home_str);
    config_dir.push(".my_pass_manager");

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!(
            "Error: Could not create config directory at {:?}: {}",
            config_dir, e
        );
        return;
    }

    let master_hash_path = config_dir.join("master.hash");
    let file_path = config_dir.join("passwords.enc");
    let salt_path = config_dir.join("salt.bin");

    let master_hash_str = master_hash_path.to_str().expect("Invalid path");
    let file_str = file_path.to_str().expect("Invalid path");
    let salt_str = salt_path.to_str().expect("Invalid path");

    let encryption_salt = storage::get_or_create_salt(salt_str);

    let mut encryption_key = if !master_hash_path.exists() {
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

            std::fs::write(master_hash_str, hash).expect("Failed to save hash");
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

        let saved_hash = std::fs::read_to_string(master_hash_str).expect("Failed to read hash");

        if !storage::verify_master_password(&input, &saved_hash) {
            input.zeroize();
            println!("Wrong Master Password! Access Denied.");
            return;
        }
        storage::derive_key(&mut input, &encryption_salt)
    };

    let mut passwords = storage::load_passwords(file_str, &encryption_key);

    match &cli.command {
        Some(Commands::Add(args)) => {
            print!("Enter password for {} : ", args.name);
            std::io::stdout().flush().expect("Flush failed");
            let mut p1 = read_password().expect("Failed to read password");

            print!("Confirm Password : ");
            std::io::stdout().flush().expect("Flush failed");
            let mut p2 = read_password().expect("Failed to read password");

            if p1 == p2 {
                passwords.insert(args.name.clone(), p1.clone());
                storage::save_passwords(file_str, &passwords, &encryption_key);
                println!("Saved successfully!");
            } else {
                println!("Passwords did not match!");
            }
            p1.zeroize();
            p2.zeroize();
        }
        Some(Commands::Get(args)) => match passwords.get(&args.name) {
            Some(pw) => println!("Password for {} : {}", args.name, pw),
            None => println!("No password found for '{}'", args.name),
        },
        Some(Commands::Delete(args)) => {
            storage::delete_password(file_str, &args.name, &encryption_key);
        }
        Some(Commands::List) => {
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
        None => unreachable!(),
    }

    encryption_key.zeroize();

    for (_, v) in passwords.iter_mut() {
        v.zeroize();
    }
}

fn print_banner() {
    let banner = r#"
        ████████  █████████████           _~^~^~_
        ▒▒███▒▒███▒▒███▒▒███▒▒███     \) /  o o  \ (/
         ▒███ ▒███ ▒███ ▒███ ▒███       '_   -   _'
         ▒███ ▒███ ▒███ ▒███ ▒███       / '-----' \
         ▒███████  █████▒███ █████
         ▒███▒▒▒  ▒▒▒▒▒ ▒▒▒ ▒▒▒▒▒
         ▒███
         █████
        ▒▒▒▒▒
        "#;
    println!("{}", banner);
}
