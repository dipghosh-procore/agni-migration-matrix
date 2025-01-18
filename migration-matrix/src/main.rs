// use clap::{Parser, Subcommand};
mod config;
mod database;

use database::tcp_conn;

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


fn main() {
    // let args = Args::parse();
    // match args.cmd {
    //     Commands::Create{db_name} =>{
    //         println!("Creating database {}", db_name);
    //     }
    // }

    let db_url = "postgres://username:password@127.0.0.1:5432/testdb";
    let app_name = "RustMigrationTool";

    tcp_conn::connect_db(db_url, app_name);

}