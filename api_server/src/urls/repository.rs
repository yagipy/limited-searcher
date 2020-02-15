#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::urls;
use urls::entity::Url;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Url>> {
    urls::table.load::<Url>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Url> {
    urls::table.find(id).get_result::<Url>(connection)
}

pub fn insert(url: Url, connection: &PgConnection) -> QueryResult<Url> {
    diesel::insert_into(urls::table)
        .values(&InsertableUrl::from_url(url))
        .get_result(connection)
}

pub fn update(id: i32, url: Url, connection: &PgConnection) -> QueryResult<Url> {
    diesel::update(urls::table.find(id))
        .set(&url)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(urls::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "urls"]
struct InsertableUrl {
    folder_id: i32,
    url: String,
    status: bool,
}

impl InsertableUrl {
    fn from_url(url: Url) -> InsertableUrl {
        InsertableUrl {
            folder_id: url.folder_id,
            url: url.url,
            status: url.status,
        }
    }
}