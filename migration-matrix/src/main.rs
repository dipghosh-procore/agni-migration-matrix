use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create{
        db_name: String,
    },
}


fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Create{db_name} =>{
            println!("Creating database {}", db_name);
        }
    }
}
