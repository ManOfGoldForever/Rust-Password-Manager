# pm 🦀

A fast, secure, and lightweight password manager CLI built entirely in Rust. No cloud, no tracking, just local encryption.

## Table of Contents

- [Why use this?](#why-use-this?)
- [Installation](#installation)
- [Build from Source](#build-from-source)
- [Uninstallation](#uninstallation)
- [Usage](#usage)
- [Security and Data](#security-and-data)
- [Future Plans](#future-plans)
- [License](#license)

### Why use this?

- **Privacy First:** Your passwords never leave your machine.
- **Modern Encryption:** Uses XChaCha20-Poly1305 and Argon2id.
- **Memory Safe:** Sensitive data is wiped from RAM immediately after use.
- **Lightweight:** Tiny binary with an instant startup and no gui bloat.

### Installation

1. Download the latest compressed folder for your OS (.tar.gz for linux/Mac or .zip for Windows) from the [releases](https://github.com/ManOfGoldForever/Rust-Password-Manager/releases/tag/first-release) tab.
2. Extract the folder
3. Run the installer (this also works when updating to a newer downloaded version):

- **Linux/Mac:** Open a terminal in the extracted folder and run: `chmod +x linux-mac-install.sh && ./linux-mac-install.sh`
- **Windows:** Right-click windows-install.ps1 and select `Run with PowerShell`

This script moves the pm binary to your "PATH" so you can use it anywhere by typing pm in a terminal.

### Build from Source

If you would rather compile the binary yourself rather than using pre-built releases, follow these steps:

1. **Prerequisites**

You will need the Rust toolchain installed.

- visit [rustup.rs](https://rustup.rs/) and follow the instructions for your OS.
  > **WARNING!** Linux users may need build essentials (like `gcc` or `make`) installed using their package manager to compile Rust crates.

2. **Clone and Build**

Use the following code in the terminal.
`git clone https://github.com/ManOfGoldForever/Rust-Password-Manager.git`
`cd Rust-Password-Manager`
`cargo build --release`

3. **Locate Binary**

After the build finishes your executable will be located at:

- **Linux/Mac:** `target/release/pm`
- **Windows:** `target/release/pm.exe`

### Uninstallation

If for whatever reason you want to remove this perfect and amazing tool along with permanently deleting your encrypted vault:

- **Linux/Mac:** Open a terminal in the extracted folder and run: `chmod +x linux-mac-uninstall.sh && ./linux-mac-uninstall.sh`
- **Windows:** Right-click windows-uninstall.ps1 and select `Run with PowerShell`

> **WARNING!** This will delete all saved passwords, your master password, and everything else along with the tool!

### Usage

The first time you run any command you'll be prompted to set a master password.

> **IMPORTANT!** The master password is the only key to access your vault so remember it and don't lose it or you won't be able to access your data.

```rust
A simple password manager CLI

Usage: pm <COMMAND>

Commands:
  add     Create a new password with a name
  get     Get an already made password using its name
  delete  Delete an already made password using its name
  list    Lists the names of all created passwords
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Security and Data

All data lives in a hidden folder in your home directory if you ever need to start from scratch you could delete it:

- **Linux/Mac:** `~/.my_pass_manager`
- **Windows:** `%USERPROFILE%\.my_pass_manager\`

This tool uses Argon2id to perform some black magic. So even if someone stole your passwords.enc file from your computer, they would not be able to read it without your master password. Basically, if someone manages to steal your passwords.enc file from your computer you have bigger problems than them figuring out your passwords.

### Future Plans

I plan to add more to this eventually like maybe a secure password generator or a TUI or something, but I don't have a list of things to add yet. Maybe after using it some I will come up with some stuff we'll see what happens.

### Contributing

If you are interested in contributing to this project look at [CONTRIBUTING](./CONTRIBUTING.md) for more information on how you can.

### License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

# Enjoy :)
