use rocket;
use connection;
use users;
use folders;
use urls;
//use searches;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/api",
           routes![
            users::service::all,
            users::service::get,
            users::service::post,
            users::service::put,
            users::service::delete,

            folders::service::all,
            folders::service::get,
            folders::service::post,
            folders::service::put,
            folders::service::delete,

            urls::service::all,
            urls::service::get,
            urls::service::post,
            urls::service::put,
            urls::service::delete,
            ],
        ).launch();
}