// use sled;
// use async_std;
use std::{collections::{HashMap}, hash::Hash};
use rusqlite::{params,Connection, Result,ToSql,Error, types::ToSqlOutput};
use rusqlite;
use std::string::String;

mod container_wrapper;
use crate::container_wrapper::{DictStrs,Vec2Str,VecStr};

use serde_json;
use serde::{Deserialize,Serialize};

#[derive(Clone,Debug,PartialEq,)]
pub struct OriginalModel {
    pub email:String,
    pub pwd:String,
    pub activation_state:String,
    pub user_type:String,
    pub history:Vec<(
	HashMap<String,String>, // information of 卦
	Vec<String>, // list of Yao
	Vec<String>, // list of xang
	String, // inps
	String, // time
	String, // place
	String, // analysis
	Vec<(String,String)>, // comment and time of res.
    )>
}

#[derive(Serialize,Deserialize)]
pub struct Account {
    pub email:String,
    pub pwd:String,
    pub activation_state:String,
    pub user_type:String,
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct EventRecord {
    // pub id: String,
    pub email:String,
    pub gua: HashMap<String,String>,
    pub yaos:Vec<String>,
    pub yaoxangs:Vec<String>,
    pub inps:String,
    pub time:String,
    pub place:String,
    pub analysis:String,
    pub comments:Vec<(String,String)>,
}

impl EventRecord{
    pub fn toSqlFormat(&self)->(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ){
        // 1. serialize `gua yaos yaoxangs comments`
        let newgua:String=serde_json::to_string(&self.gua).unwrap();
        let newyaos: String=serde_json::to_string(&self.yaos).unwrap();
        let newyxs: String=serde_json::to_string(&self.yaoxangs).unwrap();
        let newcomms: String=serde_json::to_string(&self.comments).unwrap();

        (self.email.clone(),newgua,newyaos,newyxs,
        self.inps.clone(),self.time.clone(),self.place.clone(),
        self.analysis.clone(),newcomms)
    }

    pub fn fromSql(data:(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ))->EventRecord{
        let gua:HashMap<String,String>=serde_json::from_str(&data.1).unwrap();
        let yaos:Vec<String>=serde_json::from_str(&data.2).unwrap();
        let yxs:Vec<String>=serde_json::from_str(&data.3).unwrap();
        let comments:Vec<(String,String)>=serde_json::from_str(&data.8).unwrap();
        EventRecord { email: data.0, gua: gua, yaos: yaos,
             yaoxangs: yxs, inps: data.4, time: data.5,
              place: data.6, analysis: data.7, comments: comments }
    }
}

const DB_URL:&str="../zhouyitest1.db";
const DB_NAME:&str = "zhouyi_overall";

#[derive(Debug,)]
struct DBManagement{
    DB_URL:String,
    conn:Connection,
}


impl DBManagement{

pub fn new_dbs(dbpath:&str)->DBManagement{
    let conn=Connection::open(dbpath).unwrap();
    println!("{:?}",&conn);

    conn.execute("CREATE TABLE account (
    email TEXT PRIMARY KEY,
    pwd TEXT NOT NULL,
    activation_state TEXT NOT NULL,
    user_type TEXT NOT NULL
)",
	()
    ).unwrap();

    conn.execute(&format!(
	"CREATE TABLE {} (
    id INTEGER  PRIMARY KEY,
    email TEXT NOT NULL,
    gua TEXT NOT NULL,
    yaos TEXT NOT NULL,
    yaoxangs BLOB NOT NULL,
    inps TEXT NOT NULL,
    time TEXT NOT NULL,
    place TEXT NOT NULL,
    analysis TEXT NOT NULL,
    comments TEXT NOT NULL
    )","EventRecord"),
	()
    ).unwrap();
    conn.execute("INSERT INTO account (email, pwd, activation_state, user_type) VALUES (?1, ?2,?3,?4)",
		 ("root@123.com","noyi123","activate","regular"));
    DBManagement{DB_URL:String::from(DB_URL),conn:conn}
    
}

fn open_db(url:&str)->DBManagement{
    let conn=Connection::open(url).unwrap();
    DBManagement { DB_URL: url.to_owned(), conn: conn }
}

// output code:
// "OK" -> ok
// "Email already exists."
// "Others"
pub fn create_account(&self,email:&str,pwd:&str)->String{
    // query if there exists a email
    let query=format!("SELECT * FROM account WHERE email='{}'",email);
    // let query=format!("SELECT * FROM account");
    let mut stmt=self.conn.prepare(&query).unwrap();
    if !stmt.exists([]).unwrap(){
        self.conn.execute("INSERT INTO account (email, pwd,
        activation_state, user_type) VALUES (?1, ?2, ?3, ?4)",
         [email,pwd,"not_activate","nothing"]);
        return "Ok".to_owned();
    }
    else{
        return "Email already exists.".to_owned();
    }
    
    // let acc_iter = 
    // stmt.query_map([],|row| row.get(0));
    // let mut ress=Vec::new();
    
    // for line in acc_iter{
    //     ress.push(line);
    //     println!("{:?}",&ress);
    // }

    // let length=res.query([]).unwrap();
    // println!("{:?}",length);

    // println!("{:?}",res);

    "Ok".to_owned()
}

pub fn activate_account(&self,email:&str,)->String{
    let query=format!("SELECT activation_state
     FROM account WHERE email='{}'",email);
    // let query=format!("SELECT * FROM account");
    let mut stmt=self.conn.prepare(&query).unwrap();
    if stmt.exists([]).unwrap(){

        // if already exists, give a special code.
        let mut rows=stmt.query([]).unwrap();
        let mut names:Vec<String>=Vec::new();
        let state:String=rows.next().unwrap().expect("").get(0).unwrap();
        if state.as_str()=="not_activate"{
            
            self.conn.execute("UPDATE account
            SET activation_state = 'activate'
            WHERE email = ?1",
            [email]);
            println!("activation acount finded!");
            return "Ok".to_owned();
        }
        else{
            return "Already activated.".to_owned();
        }
        
    }
    else{
        return "Email not exists.".to_owned();
    }
}

// not validated.
pub fn verify_login(&self,email:&str,pwd:&str)->(String,String,String,String){
    
    // 1. query if there exist of correct
    let sql=format!("SELECT activation_state, user_state
    FROM account
    WHERE email = '{}' AND pwd = '{}'",email,pwd);
    let mut stmt=self.conn.prepare(&sql).unwrap();
    if stmt.exists([]).unwrap(){
        let mut rows=stmt.query([]).unwrap();
        let acs:String=rows.next().unwrap().expect("").get(0).unwrap();
        let uss:String=rows.next().unwrap().expect("").get(1).unwrap();
        ("Ok".to_owned(),"1".to_owned(),acs,uss)
    }
    else{
        ("No match".to_owned(),
        "0".to_owned(),
        "not_activate".to_owned(),
        "nothing".to_owned(),
    )
    }
    // 2. return other informations.
}


pub fn get_records(&self,email:&str)->Result<Vec<EventRecord>>{

    // 1. query database
    let sql=format!("SELECT gua, yaos, yaoxangs,
     inps, time, place, analysis, comments
     FROM EventRecord
     WHERE email='{}'",email);
    //  let sql=format!("SELECT gua
    //  FROM EventRecord
    //  WHERE email='{}'",email);

    let mut stmt=self.conn.prepare(&sql).unwrap();
    
    let mut all_datas: Vec<EventRecord>=vec![];
    let mut rows=stmt.query([]).unwrap();
    while let Some(record) = rows.next().unwrap(){
        all_datas.push(
            EventRecord::fromSql
        ((
            email.to_owned(),
            record.get(0).unwrap(),
            record.get(1).unwrap(),
            record.get(2).unwrap(),
            record.get(3).unwrap(),
            record.get(4).unwrap(),
            record.get(5).unwrap(),
            record.get(6).unwrap(),
            record.get(7).unwrap(),
        )));
    }

    Ok((all_datas))
}

// pub fn merge_history(&self, email:&str)

// very straightforward, push one record up.
pub fn push_records(&self,
    data:EventRecord)->String{
        
        // 1. query database
        let email=data.email.clone();
        let time=data.time.clone();
        
        let sql=format!("SELECT id
        FROM EventRecord
        WHERE time='{}' AND email='{}'",&time,&email);
        let mut stmt=self.conn.prepare(&sql).unwrap();
        if stmt.exists([]).unwrap(){
            let item_id:i32=stmt.query([]).unwrap().next()
            .unwrap().expect("").get(0).unwrap();
            let data=EventRecord::toSqlFormat(&data);
            let params=params![
            data.1,data.2,data.3,data.4,
            data.5,data.6,data.7,data.8,item_id];
            self.conn.execute("UPDATE EventRecord
            SET gua = ?1,
            yaos = ?2,
            yaoxangs = ?3,
            inps = ?4,
            time = ?5,
            place = ?6,
            analysis = ?7,
            comments = ?8
            WHERE id= ?9", params);
            
            return "Ok".to_owned();
        }
        else{
            let data=EventRecord::toSqlFormat(&data);
            self.conn.execute("INSERT INTO EventRecord
            (email, gua, yaos, yaoxangs, inps, time, place, analysis, comments)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            [data.0,data.1,data.2,data.3,data.4,data.5,data.6,data.7,data.8]
            );

            return "Ok".to_owned();

        }
        
}


}


// // initial the database
// // #[async_std::main]
// async fn run()->Result<(),DbErr>{
//     let db: DatabaseConnection = Database::connect
//     (DB_URL).await?;
    
//     let db = &match db.get_database_backend() {
//                DbBackend::MySql => {
//                    db.execute(Statement::from_string(
//                        db.get_database_backend(),
//                        format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
//                    ))
//                    .await?;
        
//                    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
//                    Database::connect(&url).await?
//                }
//                DbBackend::Postgres => {
//                    db.execute(Statement::from_string(
//                        db.get_database_backend(),
//                        format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
//                    ))
//                    .await?;
//                    db.execute(Statement::from_string(
//                        db.get_database_backend(),
//                        format!("CREATE DATABASE \"{}\";", DB_NAME),
//                    ))
//                    .await?;
        
//                    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
//                    Database::connect(&url).await?
//                }
//                DbBackend::Sqlite => db,
//            };

//     println!("{:?}",&db);
//     Ok(())
//     // drop(newdb);
// }


fn main(){
    // let dbm= DBManagement::new_dbs("test111.db");
    let mut dbm= DBManagement::open_db("test111.db");
    let res=dbm.create_account("333root@123.com","123456");
    // let res=dbm.create_account("root111@123.com","123456");
    println!("{:?}",res);
    let res=dbm.activate_account("222root@123.com");
    println!("{:?}",res);

    // record management
    let mut record1=EventRecord::default();
    record1.email="222root@123.com".to_owned();
    
    dbm.push_records(record1);

    let mut record2=EventRecord::default();
    record2.email="222root@123.com".to_owned();
    record2.analysis="great".to_owned();
    record2.time="1111".to_owned();
    dbm.push_records(record2);

    let res=dbm.get_records("222root@123.com");
    println!("{:?}",res);
    


    }
