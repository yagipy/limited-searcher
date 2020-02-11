#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;

use dotenv::dotenv;

mod router;
mod schema;
mod connection;

mod users;
mod folders;
mod urls;

fn main() {
    dotenv().ok();
//    users::router::create_routes();
//    folders::router::create_routes();
    router::create_routes();
}

//#[catch(503)]
//fn service_not_available(_req: &Request) -> &'static str {
//    "Service is not available. (Is the database up?)"
//}

//fn main() {
//    rocket::ignite()
//        .attach(RustyDbConn::fairing())
//        .register(catchers![service_not_available])
//        .mount("/api", routes![index, search])
//        .launch();
//}
