use clap::{Parser, Subcommand};
use wa::text;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)] 
enum Commands {
    Pretty {
        raw: String,
    }
}

fn main() {
    let args = Args::parse();
    
    match args.command {
        Commands::Pretty { raw } => {
            println!("{}", text(&raw));
        },
    }
}
