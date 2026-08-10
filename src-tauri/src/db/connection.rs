use sqlx::{migrate::MigrateDatabase, Sqlite}; 
use std::time::Duration; 
use sqlx::sqlite::{SqlitePool,SqlitePoolOptions};
//use sqlx::Row;
const DB_URL: &str = "sqlite://sqlite.db"; 


pub struct Saved; 
pub struct New;

pub struct Changed;

pub async fn init_db()->anyhow::Result<()>{
    create_database().await; 
    let pool = establish_connection().await?;
    run_migrations(&pool).await?; 
    Ok(())
}



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



pub async fn establish_connection()-> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new() 
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(DB_URL)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool:&SqlitePool)-> anyhow::Result<()>{
    sqlx::migrate!("./src/db/migrations")
                .run(pool)
                .await?;
    Ok(())
}


#[tokio::test(name="db_connection_test")]
async fn db_connection_test()-> Result<(),sqlx::Error>{
    create_database().await;
    match establish_connection().await{
        Ok(pool) => {
            sqlx::migrate!("./src/db/migrations")
                .run(&pool)
                .await?;

            let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_schema
                 WHERE type='table' AND name NOT LIKE 'sqlite_%'
                 ORDER BY name"
            )
            .fetch_all(&pool).await?;
            println!("Tables in database");
            for row in rows{
                println!("- {}", row.0);
            }
            assert!(true);
        }
        Err(error) =>{
            println!("error connnecting to db: {}",error);
            assert!(false);
        }
    }

    Ok(())
}

