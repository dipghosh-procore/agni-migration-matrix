use std::net::TcpStream;
// use std::io::{Read, Write};
use crate::config::Config as Config;


pub fn start_up_protocol_message(config: &Config) -> Vec<u8> {

    let mut message = Vec::new();
    message.extend_from_slice(&[0; 4]); // add length palaceholder for messsage
    message.extend_from_slice(&(196608u32.to_be_bytes())); // Protocol version 3.0 (196608)

    let params = [
        ("user", config.user.as_str()),
        ("database", config.db_name.as_str()),
        ("application_name", config.app_name.as_str()),
    ];

    for (key, value) in params {
        message.extend_from_slice(key.as_bytes());
        message.push(0); // Null terminator
        message.extend_from_slice(value.as_bytes());
        message.push(0); // Null terminator
    }
    message.push(0); // End of key-value pairs

    let message_length = message.len() as u32;
    message[0..4].copy_from_slice(&message_length.to_be_bytes());

    message
}


pub fn handle_server_response(buffer: &[u8], conn: &mut TcpStream) -> Result<TcpStream, String> {
    let mut offset = 0;
    while offset < buffer.len() {
        let message_type = buffer[offset];
        offset += 1;

        match message_type {
            b'R' => {
                let auth_type = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
                offset += 4;

                if auth_type == 0 {
                    println!("Authentication successful.");
                } else {
                    return Err("Unsupported authentication method.".to_string());
                }
            }
            b'E' => {
                return Err(parse_error_response(&buffer[offset..]));
            }
            b'Z' => {
                println!("Server is ready for queries.");
                return Ok(conn.try_clone().unwrap());
            }
            _ => {
                println!("Unhandled message type: {}", message_type);
            }
        }
    }

    Err("Unexpected end of server response.".to_string())
}


fn parse_error_response(buffer: &[u8]) -> String {
    let mut message = String::new();
    let mut offset = 0;
    while offset < buffer.len() && buffer[offset] != 0 {
        message.push(buffer[offset] as char);
        offset += 1;
    }
    message
}