//use users;
//use rocket;
//use connection;
//
//pub fn create_routes() {
//    rocket::ignite()
//        .manage(connection::init_pool())
//        .mount("/api/users",
//               routes![
//                    users::handler::all,
//                    users::handler::get,
//                    users::handler::post,
//                    users::handler::put,
//                    users::handler::delete,
//                    ],
//        ).launch();
//}
