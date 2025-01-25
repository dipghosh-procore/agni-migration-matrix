use crate::schema_parser::schema::Schema as SchemaParser;

pub trait SqlGenerator {
    fn generate_create_table_sql(&self) -> Vec<String>;
}

impl SqlGenerator for SchemaParser {
    fn generate_create_table_sql(&self) -> Vec<String> {
        let mut schema_sql = Vec::new();

        for table in self.tables.iter() {
            schema_sql.push(table.generate_create_table_sql());
        }

        schema_sql
    }

}