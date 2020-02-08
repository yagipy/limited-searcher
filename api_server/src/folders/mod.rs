#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::folders;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "folders"]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub status: bool,
}
