**pm 🦀**

A fast, secure, and lightweight password manager CLI built entirely in Rust. No cloud, no tracking, just local encryption.

**Why use this?**

- **Privacy First:** Your passwords never leave your machine.
- **Modern Encryption:** Uses XChaCha20-Poly1305 and Argon2id.
- **Memory Safe:** Sensitive data is wiped from RAM immediately after use.
- **Lightweight:** Tiny binary with an instant startup and no gui bloat.

**Installation and Removal**

1. Download the latest compressed folder for your OS (.tar.gz for linux/Mac or .zip for Windows) from the [releases](https://github.com/ManOfGoldForever/Rust-Password-Manager/releases/tag/first-release) tab.
2. Extract the folder
3. Run the installer (this also works when updating to a newer downloaded version):

- **Linux/Mac:** Open a terminal in the extracted folder and run: '''chmod +x linux-mac-install.sh && ./linux-mac-install.sh'''
- **Windows:** Right-click windows-install.ps1 and select '''Run with PowerShell'''
  This script moves the pm binary to your PATH so you can use it anywhere by typing pm in a terminal.

**Uninstallation**

If for whatever reason you want to remove this perfect and amazing tool along with permanently deleting your encrypted vault:

- **Linux/Mac:** Open a terminal in the extracted folder and run: '''chmod +x linux-mac-uninstall.sh && ./linux-mac-uninstall.sh'''
- **Windows:** Right-click windows-uninstall.ps1 and select '''Run with PowerShell'''
  > [!WARNING] This will delete all saved passwords, your master password, and everything else along with the tool!

**Usage**

The first time you run any command you'll be prompted to set a master password.

> [!IMPORTANT] The master password is the only key to access your vault so remember it and don't lose it or you won't be able to access your data.

Enter any of the following commands into a terminal of your choice.

- **Add a password:** '''pm add github'' (Or any name you want instead of github) after this you will be prompted to enter and confirm a password.
- **Get a password:** '''pm get github''' (Or any name you want instead of github) this will print the name and password to the terminal, just a reminder Ctrl+Alt+C is how you copy from the terminal.
- **List names of saved passwords:** '''pm list''' this will list only the names of all passwords you have saved.
- **Delete an entry:** '''pm delete github''' (Or any name you want instead of github) this will permanently delete the entry with the name you type.

**Security and Data**

All data lives in a hidden folder in your home directory if you ever need to start from scratch you could delete it:

- **Linux/Mac:** '''~/.my_pass_manager'''
- **Windows:** '''%USERPROFILE%\.my_pass_manager\'''
  This tool uses Argon2id to derive your encryption key from your master password making brute-force attacks computationally expensive. So even if someone stole your passwords.enc file from your computer, they would not be able to read it without your master password. So if someone happens to steal your passwords.enc file from your computer you have bigger problems than them figuring out your passwords.

**Future Plans**

I plan to add more to this eventually like maybe a secure password generator or a TUI or something, but I don't have a list of things to add yet. Maybe after using it some I will come up with some stuff we'll see what happens.

**Enjoy :)**
