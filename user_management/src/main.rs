use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod formats;
use crate::formats::{Account,EventRecord};

mod db_management;
use crate::db_management::{DBManagement};

#[tokio::main]
async fn main() {

    let db:DBManagement=DBManagement::open_db("test111.db");

    use r2d2_sqlite::SqliteConnectionManager;
    let manager=SqliteConnectionManager::file("test111.db");
    let pool=r2d2::Pool::new(manager).unwrap();

    let app=Router::new()
    .route("/zhouyi/login", post(login))
    .route("/zhouyi/syncup", post(sync_down))
    .route("/zhouyi/syncdown", post(sync_up))
    .with_state(pool);

    let addr=SocketAddr::from(([127, 0,0,1],3933));
    // let listener=tokio::net::TcpListener::bind(addr)
    // .await.unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await.unwrap();

}

async fn login(State(pool):State<Pool<SqliteConnectionManager>>,
    account:Json<Account>) ->String{
        let inner=account.0;
        let conn=pool.get().unwrap();
        
        let email=&inner.email;
        let pwd=&inner.pwd;

        // 1. query if there exist of correct
        let sql=format!("SELECT activation_state, user_state
        FROM account
        WHERE email = '{}' AND pwd = '{}'",email,pwd);
        let mut stmt=conn.prepare(&sql).unwrap();
        if stmt.exists([]).unwrap(){
            let mut rows=stmt.query([]).unwrap();
            let acs:String=rows.next().unwrap().expect("").get(0).unwrap();
            let uss:String=rows.next().unwrap().expect("").get(1).unwrap();
            let res=("Ok".to_owned(),"1".to_owned(),acs,uss);
            format!("{}/{}/{}/{}",res.0,res.1,res.2,res.3)
        }
        else{
            let res=("No match".to_owned(),
            "0".to_owned(),
            "not_activate".to_owned(),
            "nothing".to_owned(),);
            format!("{}/{}/{}/{}",res.0,res.1,res.2,res.3)
        }
        
    }

async fn sync_down(State(pool):State<Pool<SqliteConnectionManager>>,
    email:String
)->Json<Vec<EventRecord>>{

    let conn=pool.get().unwrap();

    // 1. query database
    let sql=format!("SELECT gua, yaos, yaoxangs,
     inps, time, place, analysis, comments
     FROM EventRecord
     WHERE email='{}'",email);
    //  let sql=format!("SELECT gua
    //  FROM EventRecord
    //  WHERE email='{}'",email);

    let mut stmt=conn.prepare(&sql).unwrap();
    
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
    Json(all_datas)
}

async fn sync_up(State(pool):State<Pool<SqliteConnectionManager>>,
    record:Json<EventRecord>
)->String{

    let data=record.0;
    let conn=pool.get().unwrap();

    // 1. query database
    let email=data.email.clone();
    let time=data.time.clone();
    
    let sql=format!("SELECT id
    FROM EventRecord
    WHERE time='{}' AND email='{}'",&time,&email);
    let mut stmt=conn.prepare(&sql).unwrap();
    if stmt.exists([]).unwrap(){
        let item_id:i32=stmt.query([]).unwrap().next()
        .unwrap().expect("").get(0).unwrap();
        let data=EventRecord::toSqlFormat(&data);
        let params=params![
        data.1,data.2,data.3,data.4,
        data.5,data.6,data.7,data.8,item_id];
        conn.execute("UPDATE EventRecord
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
        conn.execute("INSERT INTO EventRecord
        (email, gua, yaos, yaoxangs, inps, time, place, analysis, comments)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        [data.0,data.1,data.2,data.3,data.4,data.5,data.6,data.7,data.8]
        );

        return "Ok".to_owned();

    }
}




