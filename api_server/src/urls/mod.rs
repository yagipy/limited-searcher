#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::urls;

pub mod handler;
pub mod repository;

use folders::Folder;

#[derive(Identifiable, Queryable, Associations, AsChangeset, Serialize, Deserialize)]
#[belongs_to(Folder)]
#[table_name = "urls"]
pub struct Url {
    pub id: i32,
    pub folder_id: i32,
    pub url: String,
    pub status: bool,
}
