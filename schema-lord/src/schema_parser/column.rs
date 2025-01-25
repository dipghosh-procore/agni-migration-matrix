use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_primary: Option<bool>,
    pub is_nullable: Option<bool>,
    pub default: Option<String>,
}


impl Column {
    pub fn generate_column_sql(&self) -> String {
        let mut column =  format!("{} {} ", self.name, self.data_type);

        if !self.is_nullable.unwrap_or(true) {
            column.push_str("NOT NULL ")
        }

        if self.is_primary.unwrap_or(false) {
            column.push_str("PRIMARY KEY ")
        }

        if let Some(default) = &self.default {
            column.push_str(format!("DEFAULT {} ", default).as_str())
        }

        column
    }
}