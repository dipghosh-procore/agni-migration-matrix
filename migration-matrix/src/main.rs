// use clap::{Parser, Subcommand};
mod config;
mod database;
mod query;

use database::tcp_conn;
use query::query_engine;
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

    let db_url = "postgresql://postgres:dipghosh099@127.0.0.1:5432/cosmos";
    let app_name = "RustMigrationTool";
    // postgresql://postgres:postgres@127.0.0.1:5432/procore_development?prepared_statements=false

    let config = Config::parse_db_url(db_url, app_name)
        .map_err(|err|  format!("Error parsing {}", err))?; 

    tcp_conn::check_config(&config); 
    // 
    let conn= tcp_conn::connect(&config).map_err(|err| format!("Error connecting to database: {}", err));
    let mut query_engine =  query_engine::QueryEngine::new(conn.unwrap());

    let queries = vec![
        "select 1 + 1"
    ];

    for query in queries {
        if let Err(err) = query_engine.send_query(query) {
            eprintln!("Query failed: {}", err);
        }
    }

    Ok(())
}