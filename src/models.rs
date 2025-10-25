use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::urls)]
pub struct NewUrl {
    pub original_url: String,
    pub short_code: String,
}
