use postgres::{Client, Error};
use async_std::fs as async_fs;  // Async file handling from async-std


fn get_migration_file_path(file_name:  &mut String) -> String {
    format!("migrate/{}.sql", file_name)  // Returns the full path to the SQL file
}


async fn apply_single_migration(client: &mut Client, file_name:  &mut String)-> Result<(), Error> {

    let file_apth = get_migration_file_path(file_name);

    let sql = async_fs::read_to_string(file_apth)
        .await
        .expect("Failed to read migration file");

    client.batch_execute(&sql)?;
    Ok(())
    

}

pub async fn apply_migration(client: &mut Client, file_name: &mut String) -> Result<(), Error> {
    apply_single_migration(client, file_name).await
}
