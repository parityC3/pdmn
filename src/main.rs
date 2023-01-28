use std::fs;
use std::fs::File;

use clap::{Args, Parser, Subcommand};
use dirs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Command
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add URL, ID, Password
    Add(AddData),
    /// Get URL, ID, Password from a keyword
    Show(Keyword),
    // Update URL, ID, Password
    Update(UpdateData),
    // Delete URL, ID, Password
    Delete(Keyword),
}

#[derive(Args)]
struct AddData {
    url: Option<String>,
    id: Option<String>,
    password: Option<String>,
}

#[derive(Args)]
struct UpdateData {
    keyword: Option<String>,
    id: Option<String>,
    password: Option<String>,
}

#[derive(Args)]
struct Keyword {
    keyword: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Password file name is "$HOME/.padmn_passwd"
    let mut file_path = dirs::home_dir().unwrap();
    file_path.push(".pdmn_passwd");

    // Create the password file if not exists
    if file_path.try_exists().is_err() {
        File::create(&file_path)?;
    }

    let contents = fs::read_to_string(file_path);
    dbg!(contents.unwrap());

    match &cli.command {
        Commands::Add(add_data) => {
            dbg!(&add_data.url, &add_data.id, &add_data.password);
        }
        Commands::Show(keyword) => {
            dbg!(&keyword.keyword);
        }
        Commands::Update(update_data) => {
            dbg!(&update_data.keyword, &update_data.id, &update_data.password);
        }
        Commands::Delete(keyword) => {
            dbg!(&keyword.keyword);
        }
    }
    Ok(())
}