use serde_json;
use std::{collections::{HashMap}, hash::Hash};
use rusqlite::{params,Connection, Result,ToSql,Error, types::ToSqlOutput};
use rusqlite::types::{FromSql,FromSqlError,FromSqlResult};

pub struct DictStrs {
    map: HashMap<String, String> 
}

impl ToSql for DictStrs {

    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
        // 序列化 map 为 Vec<u8>
        let serialized = serde_json::to_vec(&self.map).unwrap();
        let serialized=rusqlite::types::ToSql::to_sql(&serialized);
        serialized
    }
}
impl FromSql for DictStrs {

    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        let bytes = <Vec<u8> as FromSql>::column_result(value)?;
        let map: HashMap<String, String> = serde_json::from_slice(&bytes)?;

        Ok(DictStrs { map: map})
    }
}

pub struct VecStr<'a> {
    map: &'a Vec<String>
}
impl<'a> ToSql for VecStr<'a>{
    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
        // 序列化 map 为 Vec<u8>
        let serialized = serde_json::to_vec(self.map).unwrap();
        let serialized=rusqlite::types::ToSql::to_sql(&serialized);
        serialized
    }
}
impl<'a> FromSql for VecStr<'a> {

    fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, Error> {
        let bytes = <Vec<u8> as FromSql>::column_result(value)?;
        let map: HashMap<String, String> = serde_json::from_slice(&bytes)?;

        Ok(DictStrs { map: &map})
    }
}

pub struct Vec2Str<'a>{
    map: &'a Vec<(String,String)>
}
impl<'a> ToSql for Vec2Str<'a>{
    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
        // 序列化 map 为 Vec<u8>
        let serialized = serde_json::to_vec(self.map).unwrap();
        let serialized=rusqlite::types::ToSql::to_sql(&serialized);
        serialized
    }
}

impl<'a> FromSql for Vec2Str<'a> {

    fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, Error> {
        let bytes = <Vec<u8> as FromSql>::column_result(value)?;
        let map: HashMap<String, String> = serde_json::from_slice(&bytes)?;

        Ok(DictStrs { map: &map})
    }
}
