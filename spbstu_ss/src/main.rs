#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

use spbstu_ss::models::{User, NewUser, UpdatedUser};
use spbstu_ss::controllers::users_controller::*;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello? {} year old named {}!", age, name)
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let controller : UsersController = UsersController();
    return Json(controller.get_users());
}

#[post("/users", format = "json", data = "<data>")]
fn post_users(data: Json<NewUser>) -> Json<User> {
    let controller : UsersController = UsersController();
    return Json(controller.create_user(data.into_inner()));
}

#[get("/users/<user_id>")]
fn get_user(user_id: i32) -> Json<User> {
    let controller : UsersController = UsersController();
    return Json(controller.get_user(user_id))
}

#[put("/users/<user_id>", format = "json", data = "<data>")]
fn put_user(user_id: i32, data: Json<UpdatedUser>) -> Json<User> {
    let controller : UsersController = UsersController();
    return Json(controller.update_user(user_id, data.into_inner()));
}

#[delete("/users/<user_id>")]
fn delete_user(user_id: i32) -> Status {
    let controller : UsersController = UsersController();
    controller.delete_user(user_id);
    return Status::Ok;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, get_users, post_users, get_user, put_user, delete_user])
        .launch();
}