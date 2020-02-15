#![allow(proc_macro_derive_resolution_fallback)]
use schema::folders;
use users::entity::User;

#[derive(Identifiable, Queryable, Associations, AsChangeset, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "folders"]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub status: bool,
}
