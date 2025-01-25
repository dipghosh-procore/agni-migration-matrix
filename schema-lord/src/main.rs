mod schema_parser;
mod sql_generator;


use sql_generator::generator::SqlGenerator;
use crate::schema_parser::schema;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let schema = schema::Schema::parse_schema("schema.yaml")?;
    let sql_statements = schema.generate_create_table_sql();

    for sql in sql_statements {
        println!("{}", sql);
    }

    Ok(())
}
