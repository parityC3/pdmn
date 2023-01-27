use clap::{Args, Parser, Subcommand};

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


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(add_data) => {
            println!("add command: {:?} {:?} {:?}", add_data.url, add_data.id, add_data.password);
        }
        Commands::Show(keyword) => {
            println!("get command: {:?}", keyword.keyword);
        }
        Commands::Update(update_data) => {
            println!("update command: {:?} {:?} {:?}", update_data.keyword, update_data.id, update_data.password);
        }
        Commands::Delete(keyword) => {
            println!("delete command: {:?}", keyword.keyword);
        }
    }
}