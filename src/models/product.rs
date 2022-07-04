use crate::schema::products;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone)]
#[table_name = "products"]
pub struct Product {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
