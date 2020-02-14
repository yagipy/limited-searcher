#![allow(proc_macro_derive_resolution_fallback)]
use schema::folders;


#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "folders"]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub status: bool,
}
