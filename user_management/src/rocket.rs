
// #![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rocket::fairing::AdHoc;
// use rocket::response::content::Json;
use rocket::serde::json::Json;
use rocket::{Rocket, Build};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::{Debug, status::Created};
use rocket::State;


pub mod formats;
use crate::formats::{Account,EventRecord};

mod db_management;
use crate::db_management::{DBManagement};


// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[post("/login",data="<account>")]
fn login(db:State<DBManagement>,account:Json<Account>) -> &'static str {
    let inner=account.into_inner();
    
    let res=db.verify_login(&inner.email,&inner.pwd);
    format!("{}/{}/{}/{}",res.0,res.1,res.2,res.3).as_str()
}

#[post("/get_records",data="<email>")]
fn get_records(email:String)->Json<Vec<EventRecord>>{
    let res= db.get_records(&email).unwrap();
    Json(res)
}

#[post("/push_record",data="<record>")]
fn push_record(record:Json<EventRecord>)->&'static str {
    let res=db.push_records(record.into_inner()).as_str();
    res
}

#[launch]
fn rocket() -> _ {
    
    let db:DBManagement=DBManagement::open_db("test111.db");
    
    rocket::build()
	.mount("/zhouyi", routes![login,get_records,push_record])
    .state(db)
}
