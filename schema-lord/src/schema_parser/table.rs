use serde::Deserialize;
use crate::schema_parser::column::Column;
use crate::schema_parser::index::Index;

#[derive(Debug, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub indexs: Vec<Index>
}

impl Table {
    pub fn generate_create_table_sql(&self) -> String {
        let mut table_sql = format!("CREATE TABLE IF NOT EXISTS {}", self.name);
        let columns: Vec<String> = self.columns.iter().map(|column| column.generate_column_sql()).collect();
        let indexes: Vec<String> = self.indexs.iter().map(|index| index.create(&self.name)).collect();

        table_sql.push_str(&columns.join(","));
        table_sql.push_str(&indexes.join(","));

        table_sql
    }
}