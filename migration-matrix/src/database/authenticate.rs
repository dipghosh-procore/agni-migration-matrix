use crate::config::Config as Config;
use std::net::TcpStream;
use std::io::{Read, Write};
use md5::{Md5, Digest as Md5Digest};


enum AuthMethod {
    CleartextPassword,
    MD5Password(Vec<u8>), // Salt required for MD5 authentication
    Unsupported(u8),
    SuccessfulAuth,
}


pub fn send_startup_message(stream: &mut TcpStream, config: &Config)-> Result<(), String> {

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

    stream
            .write_all(&message)
            .map_err(|e| format!("Failed to send startup message: {}", e))?;
    Ok(())
}


fn parse_authen_response(buffer: &[u8]) -> Result<AuthMethod, String> {

    if buffer[0] != b'R' {
        return Err("Expected AuthenticationRequest".to_string());
    }
    let auth_type = u32::from_be_bytes(buffer[5..9].try_into().unwrap());
        match auth_type {
            0 => Ok(AuthMethod::SuccessfulAuth),
            3 => Ok(AuthMethod::CleartextPassword),
            5 => {
                let salt = buffer[9..13].to_vec();
                Ok(AuthMethod::MD5Password(salt))
            }
            other => Ok(AuthMethod::Unsupported(other as u8)),
    }
}

fn send_cleartext_password(stream: &mut TcpStream, password: &str) -> Result<(), String> {
    let mut message = Vec::new();
    message.extend(password.as_bytes());
    message.push(0x00);

    // Prefix with message type 'p' and length
    let length = (message.len() as u32 + 4).to_be_bytes();
    message.splice(0..0, length);
    message.insert(0, b'p');

    stream
        .write_all(&message)
        .map_err(|e| format!("Failed to send cleartext password: {}", e))?;
    Ok(())
}

fn send_md5_password(
    stream: &mut TcpStream,
    username: &str,
    password: &str,
    salt: &[u8],
) -> Result<(), String> {
    // Compute MD5 hash of password and username
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(username.as_bytes());
    let mut hashed = hasher.finalize_reset();

    // Compute MD5 hash of hashed password and salt
    hasher.update(&format!("{:x}", hashed));
    hasher.update(salt);
    hashed = hasher.finalize();

    // Format as md5<hash>
    let hashed_password = format!("md5{:x}", hashed);

    // Send password message
    let mut message = Vec::new();
    message.extend(hashed_password.as_bytes());
    message.push(0x00);

    // Prefix with message type 'p' and length
    let length = (message.len() as u32 + 4).to_be_bytes();
    message.splice(0..0, length);
    message.insert(0, b'p');

    stream
        .write_all(&message)
        .map_err(|e| format!("Failed to send MD5 password: {}", e))?;
    Ok(())
}

pub fn handle_authentication(stream: &mut TcpStream, config: &Config) -> Result<(), String> {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .map_err(|e| format!("Failed to read server response: {}", e))?;

    let auth_method = parse_authen_response(&buffer)?;
    match auth_method {
        AuthMethod::CleartextPassword => {
            send_cleartext_password(stream, &config.password.as_ref().unwrap().to_string())?
        }
        AuthMethod::MD5Password(salt) => {
            send_md5_password(stream, &config.user, &config.password.as_ref().unwrap().to_string(), &salt)?
        }
        AuthMethod::Unsupported(code) => {
            return Err(format!("Unsupported authentication method: {}", code));
        }
        AuthMethod::SuccessfulAuth => {
            return Ok(());
        }
    }
    Ok(())
}