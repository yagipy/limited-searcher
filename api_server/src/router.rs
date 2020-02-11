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
            users::handler::all,
            users::handler::get,
            users::handler::post,
            users::handler::put,
            users::handler::delete,

            folders::handler::all,
            folders::handler::get,
            folders::handler::post,
            folders::handler::put,
            folders::handler::delete,

            urls::handler::all,
            urls::handler::get,
            urls::handler::post,
            urls::handler::put,
            urls::handler::delete,
            ],
        ).launch();
}