use postgres::{Client, NoTls, Error};

pub fn database_connection(db_url: &str) -> Result<Client, Error> {
 let db_client =  Client::connect(db_url, NoTls)?;
 Ok(db_client)
}

pub fn create_database(client: &mut Client, db_name: &str) -> Result<(), Error> {
    let query = format!("CREATE DATABASE {}", db_name);
    client.batch_execute(&query)?;
    println!("INFO :: Creating database successful");
    Ok(())
}

