use crate::db::schema::{entries, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Debug, Serialize, Selectable, Deserialize, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

#[derive(
    Identifiable, Queryable, Serialize, Deserialize, Selectable, PartialEq, Associations, Debug,
)]
#[diesel(table_name = entries)]
#[diesel(belongs_to(User))]
pub struct Entry {
    pub id: i32,
    pub terms: String,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = entries)]
pub struct NewEntry {
    pub terms: String,
    pub user_id: i32,
}
// #[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
// pub struct CrawlResult {
//     pub id: i32,
//     pub url: String,
//     pub raw_html: String,
//     pub created_at: NaiveDateTime,
//     pub entry_id: i64,
// }
