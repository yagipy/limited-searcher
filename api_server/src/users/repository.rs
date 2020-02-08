#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::users;
use users::User;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(user))
        .get_result(connection)
}

pub fn update(id: i32, user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser {
    email: String,
    password: String,
}

impl InsertableUser {
    fn from_user(user: User) -> InsertableUser {
        InsertableUser {
            email: user.email,
            password: user.password,
        }
    }
}
