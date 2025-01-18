use crate::config::Config as Config;

pub fn connect_db(db_url: &str, app_name: &str){

    match Config::parse_db_url(db_url, app_name) {
        Ok(config) => {
            println!("Creating database {}", config.db_name);
            println!("usernae: {}", config.user);
            println!("host : {}", config.host);
            println!("port : {}", config.port);
            println!("app_name : {}", config.app_name);

            match config.password {
                Some(password) => println!("password : {}", password),
                None => println!("password none")
            }

        }
        Err(err) => eprintln!("Error creating database {}", err)
    }
}