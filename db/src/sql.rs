use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use log::{info};


pub async fn create_sql_db() -> Result<DatabaseConnection, DbErr> {
    info!("creating sql db!");
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

   let db = &match db.get_database_backend() {
       DbBackend::MySql => {
           db.execute(Statement::from_string(
               db.get_database_backend(),
               format!("CREATE DATABASE IF NOT EXISTS `{}`;", std::env::var("MYSQL_DATABASE").unwrap()),
           ))
           .await?;

           let url = format!("{}/{}", std::env::var("DATABASE_URL").unwrap(), std::env::var("MYSQL_DATABASE").unwrap());
           Database::connect(&url).await?
       }
       _ => panic!("only mysql is supported for this example.")
    };

    info!("running migrations!");
    Migrator::refresh(db).await?;
    info!("db created successfully :D");
    Ok::<DatabaseConnection, DbErr>(db.clone())
}
