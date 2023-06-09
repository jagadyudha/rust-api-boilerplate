use crate::schema::users;
use chrono::NaiveDate;
use diesel::prelude::*;
use rocket::FromForm;
use serde::{Serialize, Serializer};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    #[serde(serialize_with = "serialize_naive_date")]
    pub created_at: NaiveDate,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub avatar: &'a str,
}

#[derive(Insertable, FromForm, AsChangeset)]
#[diesel(table_name = users)]
pub struct EditUser<'a> {
    pub name: &'a str,
    pub avatar: &'a str,
}

fn serialize_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str: String = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&date_str)
}
