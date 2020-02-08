#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::folders;
use folders::Folder;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Folder>> {
    folders::table.load::<Folder>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Folder> {
    folders::table.find(id).get_result::<Folder>(connection)
}

pub fn insert(folder: Folder, connection: &PgConnection) -> QueryResult<Folder> {
    diesel::insert_into(folders::table)
        .values(&InsertableFolder::from_folder(folder))
        .get_result(connection)
}

pub fn update(id: i32, folder: Folder, connection: &PgConnection) -> QueryResult<Folder> {
    diesel::update(folders::table.find(id))
        .set(&folder)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(folders::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "folders"]
struct InsertableFolder {
    name: String,
    status: bool,
}

impl InsertableFolder {
    fn from_folder(folder: Folder) -> InsertableFolder {
        InsertableFolder {
            name: folder.name,
            status: folder.status,
        }
    }
}
