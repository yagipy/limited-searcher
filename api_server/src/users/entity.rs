#![allow(proc_macro_derive_resolution_fallback)]
use schema::users;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}
