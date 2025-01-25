use serde::Deserialize;


// CREATE INDEX index_name
// ON table_name (column1, column2, column3)

#[derive(Debug, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: Option<bool>,
}

impl Index {

    pub fn create(&self, table_name: &str) -> String {
        let unique = if self.unique.unwrap_or(false) { "UNIQUE" } else { "" };
        let index = format!("CREATE {} INDEX {} ON {} ({})", self.name, unique, table_name, self.columns.join(","));
        index
    }
}