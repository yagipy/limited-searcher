//use folders;
//use rocket;
//use connection;
//
//pub fn create_routes() {
//    rocket::ignite()
//        .manage(connection::init_pool())
//        .mount("/api/folders",
//               routes![
//                    folders::handler::all,
//                    folders::handler::get,
//                    folders::handler::post,
//                    folders::handler::put,
//                    folders::handler::delete,
//                    ],
//        ).launch();
//}
