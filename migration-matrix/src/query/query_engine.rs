use std::net::TcpStream;
use std::io::{Read, Write};


pub struct QueryEngine {
    stream: TcpStream,
}


impl QueryEngine {

    pub fn new(stream: TcpStream) -> QueryEngine {
        QueryEngine {
            stream
        }
    }

    pub fn send_query(&mut self, query: &str) -> Result<(), String> {
        println!("Executing query: {}", query);

        // Step 1: Create the message
        let mut message = Vec::new();
        message.push(b'Q'); // 'Q' indicates a Query command
        let length = (query.len() + 5) as u32; // Length includes query + 4 bytes for length + 1 null byte
        message.extend(&length.to_be_bytes()); // Add the length in big-endian format
        message.extend(query.as_bytes()); // Add the query bytes
        message.push(0x00); // Null-terminate the query

        println!("Query message: {:?}", message);
        println!("Query message: {}", length);

        self.stream
                .write_all(&message)
                .map_err(|e| format!("Failed to send query message: {}", e))?;


        // Read server response message
        let mut buffer = [0; 1024];
        let bytes_read = self
            .stream
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read query response: {}", e))?;

        self.parse_response(&buffer[..bytes_read])?;

        Ok(())
    }

    fn parse_response(&self, response: &[u8]) -> Result<(), String> {
        if response.is_empty() {
            return Err("Empty response from server".to_string());
        }
    
        let mut offset = 0;
        while offset < response.len() {
            match response[offset] {
                b'T' => {
                    // Row Description
                    offset += self.parse_row_description(&response[offset..])?;
                }
                b'D' => {
                    // Data Row
                    offset += self.parse_data_row(&response[offset..])?;
                }
                b'C' => {
                    // Command Complete
                    let message = String::from_utf8_lossy(&response[offset + 5..response.len() - 1]);
                    println!("Query executed successfully: {}", message);
                    break;
                }
                b'E' => {
                    // Error Response
                    let message = String::from_utf8_lossy(&response[offset + 5..response.len() - 1]);
                    return Err(format!("Error from server: {}", message));
                }
                _ => {
                    println!("Unhandled response type: {}", response[offset]);
                    offset += 1; // Skip unknown response
                }
            }
        }
    
        Ok(())
    }


    fn parse_row_description(&self, response: &[u8]) -> Result<usize, String> {
        if response[0] != b'T' {
            return Err("Invalid Row Description response".to_string());
        }
    
        let field_count = u16::from_be_bytes([response[5], response[6]]);
        println!("Row Description: {} fields", field_count);
    
        let mut offset = 7;
        for _ in 0..field_count {
            // Read field name (null-terminated string)
            let end = response[offset..]
                .iter()
                .position(|&b| b == 0)
                .ok_or("Invalid Row Description: missing null terminator")?;
            let field_name = String::from_utf8_lossy(&response[offset..offset + end]);
            offset += end + 1; // Move past the null terminator
    
            // Skip field metadata (18 bytes: table ID, column ID, type ID, etc.)
            offset += 18;
    
            println!("Field: {}", field_name);
        }
    
        Ok(offset)
    }

    fn parse_data_row(&self, response: &[u8]) -> Result<usize, String> {
        if response[0] != b'D' {
            return Err("Invalid Data Row response".to_string());
        }

        let column_count = u16::from_be_bytes([response[5], response[6]]);
        println!("Data Row: {} columns", column_count);

        let mut offset = 7;
        for _ in 0..column_count {
            let col_len = i32::from_be_bytes([
                response[offset],
                response[offset + 1],
                response[offset + 2],
                response[offset + 3],
            ]);
            offset += 4;

            if col_len == -1 {
                println!("Column: NULL");
            } else {
                let value = String::from_utf8_lossy(&response[offset..offset + col_len as usize]);
                println!("Column: {}", value);
                offset += col_len as usize;
            }
        }

        Ok(offset)
    }
    
    
}