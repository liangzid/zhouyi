// use sled;
// use async_std;
use std::collections::{HashMap};
use sea_orm::entity::prelude::*;
use sea_orm::{ConnectionTrait,DbBackend,Statement,DatabaseConnection,Database,DbErr};
use futures::executor::block_on;

mod migrator;

const DB_URL:&str="sqlite:./sqlite.db?mode=rwc";
const DB_NAME:&str = "zhouyi_overall";

// #[derive(Clone,Debug,PartialEq,DeriveEntityModel)]
// #[sea_orm(table_name = "zhouyi_record")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub email:String,
//     pub pwd:String,
//     pub activation_state:String,
//     pub user_type:String,
//     pub history:Vec<(
// 	HashMap<String,String>,
// 	Vec<String>,
// 	Vec<String>,
// 	String,
// 	String,
// 	String,
// 	String,
// 	Vec<(String,String)>,
//     )>
// }

// initial the database
// #[async_std::main]
async fn run()->Result<(),DbErr>{
    let db: DatabaseConnection = Database::connect
    (DB_URL).await?;
    
    let db = &match db.get_database_backend() {
               DbBackend::MySql => {
                   db.execute(Statement::from_string(
                       db.get_database_backend(),
                       format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
                   ))
                   .await?;
        
                   let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                   Database::connect(&url).await?
               }
               DbBackend::Postgres => {
                   db.execute(Statement::from_string(
                       db.get_database_backend(),
                       format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
                   ))
                   .await?;
                   db.execute(Statement::from_string(
                       db.get_database_backend(),
                       format!("CREATE DATABASE \"{}\";", DB_NAME),
                   ))
                   .await?;
        
                   let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                   Database::connect(&url).await?
               }
               DbBackend::Sqlite => db,
           };

    println!("{:?}",&db);
    Ok(())
    // drop(newdb);
}


fn main(){
    if let Err(err)=block_on(run()){
        panic!("{}",err);
    }
}
