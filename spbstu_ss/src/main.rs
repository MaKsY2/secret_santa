#![feature(proc_macro_hygiene, decl_macro)]

use diesel::result::Error;
use rocket::*;
use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

use spbstu_ss::models::user_model::{User, NewUser, UpdatedUser};
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
fn get_user(user_id: i32) -> Result<Json<User>, NotFound<String>> {
    let controller : UsersController = UsersController();
    return match controller.get_user(user_id) {
        Ok(user) => Ok(Json(user)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(NotFound(err.to_string())) } else { panic!("{}", err.to_string()) }
    };
}

#[put("/users/<user_id>", format = "json", data = "<data>")]
fn put_user(user_id: i32, data: Json<UpdatedUser>) -> Result<Json<User>, NotFound<String>> {
    let controller : UsersController = UsersController();
    return match controller.update_user(user_id, data.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(NotFound(err.to_string())) } else { panic!("{}", err.to_string()) }
    }
}

#[delete("/users/<user_id>")]
fn delete_user(user_id: i32) -> Result<Status, NotFound<String>> {
    let controller : UsersController = UsersController();
    return match controller.delete_user(user_id) {
        Ok(_res) => if _res == 0 {Err(NotFound("Not found".to_string()))} else {Ok(Status::Ok)},
        Err(err) => panic!("{}", err.to_string())
    };
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, get_users, post_users, get_user, put_user, delete_user])
        .launch();
}