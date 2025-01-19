use crate::config::Config as Config;
use std::net::TcpStream;
// use crate::database::protocol;
use crate::database::authenticate;


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

pub fn connect(config: &Config) -> Result<(), String> {

    let mut conn = TcpStream::connect((config.host.clone(), config.port)).map_err(|e| format!("failed to connect to {}", e))?;
    println!("Connected to {}:{}", config.host, config.port);
    // Ok(())
    authenticate::send_startup_message(&mut conn, &config)?;
    // Handle authentication
    authenticate::handle_authentication(&mut conn, &config)?;
    Ok(())

}