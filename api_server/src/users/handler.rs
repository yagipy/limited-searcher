use connection::DbConn;
use diesel::result::Error;
use std::env;
use users;
use users::entity::User;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/users")]
pub fn all(connection: DbConn) -> Result<Json<Vec<User>>, Status> {
    users::repository::all(&connection)
        .map(|users| Json(users))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/users/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    users::repository::get(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[post("/users", format = "application/json", data = "<user>")]
pub fn post(user: Json<User>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
    users::repository::insert(user.into_inner(), &connection)
        .map(|user| user_created(user))
        .map_err(|error| error_status(error))
}

fn user_created(user: User) -> status::Created<Json<User>> {
    status::Created(
        format!("{host}:{port}/api/users/{id}", host = host(), port = port(), id = user.id).to_string(),
        Some(Json(user)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/users/<id>", format = "application/json", data = "<user>")]
pub fn put(id: i32, user: Json<User>, connection: DbConn) -> Result<Json<User>, Status> {
    users::repository::update(id, user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[delete("/users/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match users::repository::get(id, &connection) {
        Ok(_) => users::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
