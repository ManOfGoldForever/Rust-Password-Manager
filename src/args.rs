use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PassManager")]
#[command(about = " A simple password manager CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new password with a name
    Add(AddPass),
    /// Get an already made password using its name
    Get(GetPass),
    /// Delete an already made password using its name
    Delete(DeletePass),
    /// Lists the names of all created passwords
    List,
    /// Internal use only: clears clipboard after delay
    #[command(hide = true)]
    CleanupClipboard { delay: u64 },
}

#[derive(Args)]
pub struct AddPass {
    /// The name of the password to add
    pub name: String,
}

#[derive(Args)]
pub struct GetPass {
    /// The name of the password to get
    pub name: String,
    /// Copy the password to the clipboard instead of printing it
    #[arg(short, long)]
    pub copy: bool,
}

#[derive(Args)]
pub struct DeletePass {
    /// The name of the password to delete
    pub name: String,
}
