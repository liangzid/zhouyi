use std::{collections::HashMap, hash::Hash, f64::consts::E};
use reqwest;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug,Default)]
struct Account {
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

pub async fn signup(email:&str, pwd:&str)->(String,String,String,String){
    let a=Account{email:email.to_owned(),pwd:pwd.to_owned(),
        activation_state:"not_activate".to_owned(),
        user_type:"nothing".to_owned(),
    };
    let cl=reqwest::Client::new();
    // let res=cl.post("http://localhost:3933/zhouyi/signup")
    let res=cl.post("http://viceversaai.icu:3933/zhouyi/signup")
    .json(&a)
    .send()
    .await.unwrap().text().await.unwrap();
    println!("--->{:?}",&res);
    let v:Vec<&str>=res.as_str().split('/').collect();
    (v.get(0).unwrap().to_owned().to_owned(),
    v.get(1).unwrap().to_owned().to_owned(),
    v.get(2).unwrap().to_owned().to_owned(),
    v.get(3).unwrap().to_owned().to_owned())

}

pub async fn activate(email:&str)->String{
    let cl=reqwest::Client::new();
    // let res=cl.post("http://localhost:3933/zhouyi/activate")
	let res=cl.post("http://viceversaai.icu:3933/zhouyi/activate")
    .body(email.to_owned())
	.send()
	.await.unwrap().text().await.unwrap();
    res
}

// query and return the login state
pub async fn query_login(email:&str, pwd:&str)->(String,String,String,String){
    let a=Account{email:email.to_owned(),pwd:pwd.to_owned(),
        activation_state:"not_activate".to_owned(),
        user_type:"nothing".to_owned(),
    };
    let cl=reqwest::Client::new();
    // let res=cl.post("http://localhost:3933/zhouyi/login")
    let res=cl.post("http://viceversaai.icu:3933/zhouyi/login")
    .json(&a)
    .send()
    .await.unwrap().text().await.unwrap();
    println!("--->{:?}",&res);
    let v:Vec<&str>=res.as_str().split('/').collect();
    (v.get(0).unwrap().to_owned().to_owned(),
    v.get(1).unwrap().to_owned().to_owned(),
    v.get(2).unwrap().to_owned().to_owned(),
    v.get(3).unwrap().to_owned().to_owned())
}

// appended the new history information into clouds
pub fn merge_history(email:&String,historys:Vec<(
    HashMap<String,String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    String,
    String,
    Vec<(String,String)>,
)>)->String{

    "Ok".to_owned()
}


pub async fn push_record(record:EventRecord)->String{
    // let e_s=serde_json::to_string(&record).unwrap();
    let cl=reqwest::Client::new();
    // let res=cl.post("http://localhost:3933/zhouyi/pushup")
    let res=cl.post("http://viceversaai.icu:3933/zhouyi/pushup")
    .json(&record)
    .send()
    .await.unwrap().text().await.unwrap();
    res
}

pub async fn merge_records(email:&str, historys:&Vec<(
    HashMap<String,String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    String,
    String,
    Vec<(String,String)>,
)>)->String{
    for his in historys{
        // if there exist this records:
        let eventrecord=EventRecord{
            email:email.to_owned(),
            gua:his.0.clone(),
            yaos:his.1.clone(),
            yaoxangs:his.2.clone(),
            inps:his.3.clone(),
            time:his.4.clone(),
            place:his.5.clone(),
            analysis:his.6.clone(),
            comments:his.7.clone()
        };
        let res=push_record(eventrecord).await;
        println!("{:?}",res);
    }
    "Ok".to_owned()
}

// sync the history info from server to local devices(email)
pub async fn get_history(email:&str)->Vec<(
    HashMap<String,String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    String,
    String,
    Vec<(String,String)>,
)>{
    let cl=reqwest::Client::new();
    // let res:Vec<EventRecord>=cl.post("http://localhost:3933/zhouyi/tolocal")
    let res:Vec<EventRecord>=cl.post("http://viceversaai.icu:3933/zhouyi/tolocal")
    .body(email.to_owned())
    .send()
    .await.unwrap()
    .json().await.unwrap();

    // let datas:Vec<EventRecord> =serde_json::from_str(&res).unwrap();
    let mut ds: Vec<(HashMap<String, String>, Vec<String>, Vec<String>, String, String, String, String, Vec<(String, String)>)>=vec![];
    for d in res{
        ds.push((d.gua,d.yaos,d.yaoxangs,d.inps,d.time,d.place,d.analysis,d.comments));
    }
    ds
}


fn main(){
    let rt=tokio::runtime::Builder::new_current_thread()
                    .enable_all().build().unwrap();
    rt.block_on(async{
        let res=query_login("helloworld@123.com","111").await;
        println!("{:?}",res);
        let res=get_history("222root@123.com",).await;
        println!("{:?}",res);


        let mut er:EventRecord=EventRecord::default();
        er.email="222root@123.com".to_owned();
        er.time="2023".to_owned();
        er.place="here".to_owned();

        let res=push_record(er).await;
        println!("{:?}",res);
    });


    
}
