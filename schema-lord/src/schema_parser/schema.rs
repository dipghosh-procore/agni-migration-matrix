use serde::Deserialize;
use crate::schema_parser::table::Table;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub tables: Vec<Table>,
}

impl Schema {
    pub fn parse_schema(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let schema: Self = serde_yaml::from_str(&file_content)?;
        Ok(schema)
    }
}