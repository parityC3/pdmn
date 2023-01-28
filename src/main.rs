use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Password file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
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


fn main() {
    let cli = Cli::parse();

    let file_path = match cli.file {
        Some(f) => f,
        None => PathBuf::from(r"~/.passwd"),
    };
    dbg!(file_path);

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
}