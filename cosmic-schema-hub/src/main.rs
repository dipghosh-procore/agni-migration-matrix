mod config;
mod db;
mod migrate;
use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;



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

    Drop{
        db_name: String,
    },
    Migrate{
        filename: String,
    }
}




fn main() {
    config::load_config();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client: postgres::Client = db::database_connection(&db_url).expect("Failed to connect to database");


    let args = Args::parse();

    match args.cmd {

        Commands::Create{db_name} => {
            println!("Creating database: {}", db_name);
            db::create_database(&mut client, db_name).expect("Failed to create database");
        },
        Commands::Drop { db_name } => {
            println!("Droping database: {}", db_name);
            db::drop_database(&mut client, db_name).expect("Failed to drop database");
        }

        Commands::Migrate { filename } => {
            println!("{}", filename);
            let rt = Runtime::new().unwrap();
            let mut filename = filename.clone();

            rt.block_on(migrate::apply_migration(&mut client,  &mut filename))
            .expect("Failed to apply migration");

        }
    }
}
