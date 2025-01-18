// use clap::{Parser, Subcommand};
mod config;
mod database;

use database::tcp_conn;
use config::Config as Config;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[command(subcommand)]
//     cmd: Commands
// }

// #[derive(Subcommand, Debug, Clone)]
// enum Commands {
//     Connect{
//         db_name: String,
//     },
// }




fn main() -> Result<(), String> {
    // let args = Args::parse();
    // match args.cmd {
    //     Commands::Create{db_name} =>{
    //         println!("Creating database {}", db_name);
    //     }
    // }

    let db_url = "postgres://username:password@127.0.0.1:5432/testdb";
    let app_name = "RustMigrationTool";

    let config = Config::parse_db_url(db_url, app_name)
        .map_err(|err|  format!("Error parsing {}", err))?; 

    tcp_conn::check_config(&config); 
    match tcp_conn::connect(&config) {
        Ok(_) => println!("Database connection established successfully."),
        Err(err) => eprintln!("Error: {}", err),
    }
    
    Ok(())
}