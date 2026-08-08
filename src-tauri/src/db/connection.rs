use sqlx::{migrate::MigrateDatabase, Sqlite}; 

const DB_URL: &str = "sqlite://sqlite.db"; 
 
pub async fn create_database(){
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false){
        println!("Creating database {}",DB_URL);
        match Sqlite::create_database(DB_URL).await{
            Ok(_) => println!("Create db success"), 
            Err(error) => panic!("error: {}",error),
        }
    } else {
        println!("Database already exists");
    }
}

#[tokio::test(name="db_connection_test")]
async fn db_connection_test(){
    create_database().await;
    assert!(true);
}