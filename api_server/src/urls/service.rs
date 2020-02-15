use connection::DbConn;
use diesel::result::Error;
use std::env;
use urls;
use urls::entity::Url;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/urls")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Url>>, Status> {
    urls::repository::all(&connection)
        .map(|urls| Json(urls))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/urls/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Url>, Status> {
    urls::repository::get(id, &connection)
        .map(|url| Json(url))
        .map_err(|error| error_status(error))
}

#[post("/urls", format = "application/json", data = "<url>")]
pub fn post(url: Json<Url>, connection: DbConn) -> Result<status::Created<Json<Url>>, Status> {
    urls::repository::insert(url.into_inner(), &connection)
        .map(|url| url_created(url))
        .map_err(|error| error_status(error))
}

fn url_created(url: Url) -> status::Created<Json<Url>> {
    status::Created(
        format!("{host}:{port}/api/urls/{id}", host = host(), port = port(), id = url.id).to_string(),
        Some(Json(url)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/urls/<id>", format = "application/json", data = "<url>")]
pub fn put(id: i32, url: Json<Url>, connection: DbConn) -> Result<Json<Url>, Status> {
    urls::repository::update(id, url.into_inner(), &connection)
        .map(|url| Json(url))
        .map_err(|error| error_status(error))
}

#[delete("/urls/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match urls::repository::get(id, &connection) {
        Ok(_) => urls::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
