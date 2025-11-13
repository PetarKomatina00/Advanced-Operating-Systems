use crate::schema::lekovi;
use bigdecimal::BigDecimal;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Clone)]
pub struct Lek {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = lekovi)]
pub struct NoviLek {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
}
