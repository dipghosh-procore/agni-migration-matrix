use crate::config::Config as Config;
use std::net::TcpStream;
use crate::database::protocol;
use std::io::{Read, Write};


pub fn check_config(config: &Config) {

    println!("Creating database {}", config.db_name);
    println!("usernae: {}", config.user);
    println!("host : {}", config.host);
    println!("port : {}", config.port);
    println!("app_name : {}", config.app_name);

    match &config.password {
        Some(password) => println!("password : {}", password),
        None => println!("password none")
    }
}

pub fn connect(config: &Config) -> Result<TcpStream, String> {

    let mut conn = TcpStream::connect((config.host.clone(), config.port)).map_err(|e| format!("failed to connect to {}", e))?;
    println!("Connected to {}:{}", config.host, config.port);
    // Ok(())

    let start_mesage = protocol::start_up_protocol_message(&config);
    conn.write_all(&start_mesage).map_err(|e| format!("Failed to send startup message: {}", e))?;

    let mut buffer = [0; 1024];
    let bytes_read = conn
        .read(&mut buffer)
        .map_err(|e| format!("Failed to read server response: {}", e))?;

    protocol::handle_server_response(&buffer[..bytes_read], &mut conn)
}