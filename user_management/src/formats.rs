use std::{collections::{HashMap}, hash::Hash};
use std::string::String;
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

#[derive(Serialize,Deserialize,Debug,Default)]
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
