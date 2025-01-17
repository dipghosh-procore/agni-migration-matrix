mod config;
mod db;
use clap::{Arg, Command};

fn main() {
    config::load_config();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = db::database_connection(&db_url).expect("Failed to connect to database");

    let matches = Command::new("cosmic-schema-hub")
        .version("0.1")
        .author("Your Name")
        .about("cosmic schema migrator for AFM")
        .subcommand(
            Command::new("create-db")
                .about("Create the database")
        ).subcommand(
            Command::new("drop-db")
                .about("Drop the database")
        ).subcommand(
            Command::new("migrate")
                .about("Apply migrations")
                .arg(
                    Arg::new("folders")
                        .long("folders")
                        .help("Migration folders to apply"),
                )
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("create-db") {
        db::create_database(&mut client, "your_database_name").expect("Failed to create database");
    }
}
