use crate::schema::measurements;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone)]
pub struct Measurement {
    pub id: String,
    pub name: String,
    pub short_name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub short_name: &'a str,
}
