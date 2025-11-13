use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name=crate::schema::events)]
pub struct Events {
    pub id: i32,
    pub event_type: String,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::events)]
pub struct NewEvent{
    pub event_type: String,
    pub name: String,
}