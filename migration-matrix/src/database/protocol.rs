use std::net::TcpStream;
use crate::config::Config as Config;


fn reconstruct_message(buffer: &[u8]) -> String {
    // Convert each byte to a character and collect them into a string
    buffer.iter().map(|&byte| {
        if byte == 0 {
            ' '  // Handle null bytes as spaces or use something like '\0'
        } else {
            byte as char
        }
    }).collect()
}



pub fn start_up_protocol_message(config: &Config) -> Vec<u8> {

    let mut message = Vec::new();
    message.extend_from_slice(&[0; 4]); // add length palaceholder for messsage
    message.extend_from_slice(&(196608u32.to_be_bytes())); // Protocol version 3.0 (196608)
    let params = [
        ("user", config.user.as_str()),
        ("database", config.db_name.as_str()),
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
    let mut accumulated_message = Vec::new(); 

    while offset < buffer.len() {
        let message_type = buffer[offset];
        offset += 1;

        match message_type {
            b'R' => {
                let auth_type = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
                offset += 4;

                println!("auth type {}", auth_type);

                match auth_type {
                    0 => println!("Authentication successful."),
                    5 => {
                        println!("MD5 authentication required.");
                        // Handle MD5 authentication
                    },
                    8 => {
                        println!("cleartxt password is required");
                    }
                    10 => {
                        println!("SCRAM-SHA-256 authentication required.");
                    },
                    _ => return Err(format!("Unsupported authentication method: {}", auth_type)),
                }
            }
            b'A' => {  // Authentication Message (AM-SHA-256)
                let auth_message = String::from_utf8_lossy(&buffer[offset..]);
                println!("Received authentication message: {}", auth_message);
                // Handle AM-SHA-256 specifics here (SCRAM)
            }
            b'E' => {
                return Err(parse_error_response(&buffer[offset..]));
            }
            b'Z' => {
                println!("Server is ready for queries.");
                if !accumulated_message.is_empty() {
                    let message = reconstruct_message(&accumulated_message);
                    println!("Reconstructed message: {}", message);
                }

                return Ok(conn.try_clone().unwrap());
            }
            _ => {
                // println!("Unhandled message type: {}", message_type);
                accumulated_message.push(message_type);
            }
        }
    }


    if !accumulated_message.is_empty() {
        let message = reconstruct_message(&accumulated_message);
        println!("Reconstructed message: {}", message);
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