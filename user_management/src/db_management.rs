use sled;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection,Database};

#[derive(Clone,Debug,PartialEq,DeriveEntityModel)]
#[sea_orm(table_name = "")]
pub struct Zhouyi_Record {
    #[sea_orm(primary_key)]
    pub email:String,
    pub pwd:String,
    pub activation_state:String,
    pub user_type:String,
    pub history:Vec<(
	HashMap<String,String>,
	Vec<String>,
	Vec<String>,
	String,
	String,
	String,
	String,
	Vec<(String,String)>,
    )>
}

// initial the database
pub fn main(){
    let newdb=sled::open("../db/data.sled").unwrap();
    let db: DatabaseConnection = Database::connect("sqlite://username:password@host/database").await?;
    
    println!("{:?}",&newdb);
    // drop(newdb);
}
