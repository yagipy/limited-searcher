use connection::DbConn;
use diesel::result::Error;
use std::env;
use folders;
use folders::entity::Folder;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/folders")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Folder>>, Status> {
    folders::repository::all(&connection)
        .map(|folders| Json(folders))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/folders/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Folder>, Status> {
    folders::repository::get(id, &connection)
        .map(|folder| Json(folder))
        .map_err(|error| error_status(error))
}

#[post("/folders", format = "application/json", data = "<folder>")]
pub fn post(folder: Json<Folder>, connection: DbConn) -> Result<status::Created<Json<Folder>>, Status> {
    folders::repository::insert(folder.into_inner(), &connection)
        .map(|folder| folder_created(folder))
        .map_err(|error| error_status(error))
}

fn folder_created(folder: Folder) -> status::Created<Json<Folder>> {
    status::Created(
        format!("{host}:{port}/api/folders/{id}", host = host(), port = port(), id = folder.id).to_string(),
        Some(Json(folder)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/folders/<id>", format = "application/json", data = "<folder>")]
pub fn put(id: i32, folder: Json<Folder>, connection: DbConn) -> Result<Json<Folder>, Status> {
    folders::repository::update(id, folder.into_inner(), &connection)
        .map(|folder| Json(folder))
        .map_err(|error| error_status(error))
}

#[delete("/folders/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match folders::repository::get(id, &connection) {
        Ok(_) => folders::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
