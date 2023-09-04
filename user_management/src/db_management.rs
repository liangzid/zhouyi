// use sled;
// use async_std;
use std::collections::{HashMap};
use rusqlite::{params,Connection, Result};


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


pub struct Account {
    pub email:String,
    pub pwd:String,
    pub activation_state:String,
    pub user_type:String,
}

pub struct EventRecord {
    pub id: String,
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


const DB_URL:&str="../zhouyitest1.db";
const DB_NAME:&str = "zhouyi_overall";

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
    gua BLOB NOT NULL,
    yaos BLOB NOT NULL,
    yaoxangs BLOB NOT NULL,
    inps TEXT NOT NULL,
    time TEXT NOT NULL,
    place TEXT NOT NULL,
    analysis TEXT NOT NULL,
    comments BLOB NOT NULL
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
pub fn create_account(self,email:&str,pwd:&str)->String{
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

pub fn activate_account(self,email:&str,)->String{
    
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
    let dbm= DBManagement::open_db("test111.db");
    let res=dbm.create_account("222root@123.com","123456");
    // let res=dbm.create_account("root111@123.com","123456");
    
    println!("{:?}",res);
}
