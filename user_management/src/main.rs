
#![feature(decl_macro)]
#[macro_use] extern crate rocket;
// use rocket::response::content::Json;
use rocket::serde::json::Json;

mod formats;
use crate::formats::{Account,EventRecord};

mod db_management;
use crate::db_management::{DBManagement};

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[post("/login",data="<account>")]
fn login(db:&DBManagement,account:Json<Account>) -> &'static str {
    let inner=account.into_inner();
    
    let res=db.verify_login(inner.email,inner.pwd);
    format!("{}/{}/{}/{}",res.0,res.1,res.2,res.3).as_str()
}

#[post("/get_records",data="<email>")]
fn get_records(db:&DBManagement,email:String)->Json<Vec<EventRecord>>{
    let res= db.get_records(&email).unwrap();
    Json(res)
}

#[post("/push_record",data="<record>")]
fn push_record(db:&DBManagement, record:Json<EventRecord>)->&'static str {
    let res=db.push_records(record).as_str();
    res
}

#[launch]
fn rocket() -> _ {
    rocket::ignite()
	.mount("/zhouyi", routes![login,get_records,push_record])
	.launch();
}
