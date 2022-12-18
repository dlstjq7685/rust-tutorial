use std::{future::Future, time::Duration, error::Error};

use sea_orm::{DatabaseConnection, Database, DbErr, ConnectOptions};

const DB_STRING: &str = "sqlite://test.db";

//https://www.sea-ql.org/SeaORM/docs/0.9.x/write-test/sqlite/
pub fn call() -> Result<(), Box<dyn Error>> {
    println!("db api module call");
    println!();
    println!();
    println!();
    
    async {
        sea_orm_connect().await;
    };
    
    Ok(())    
}

// sqlite use
async fn sea_orm_connect() -> Result<DatabaseConnection, DbErr> {
    
    let mut opt = ConnectOptions::new(DB_STRING.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    
    let db: DatabaseConnection = Database::connect(opt).await?;

    println!("db connect");
    Ok(db)
}
