#![feature(proc_macro_hygiene, decl_macro)]

mod auth;

use diesel::result::Error;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::*;
use rocket_contrib::json::Json;
use std::convert::Infallible;

use auth::*;
use spbstu_ss::controllers::users_controller::*;
use spbstu_ss::models::user_model::{NewUser, UpdatedUser, User};
use spbstu_ss::models::{Claims, LoginRequest, LoginResponse};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello? {} year old named {}!", age, name)
}

#[post("/login", format = "json", data = "<name>")]
pub fn login(name: Json<LoginRequest>) -> Result<Json<LoginResponse>, NotFound<String>> {
    let controller: UsersController = UsersController();
    let user_res = controller.get_user_by_name(name.into_inner().name.clone());
    return match user_res {
        Ok(user) => Ok(Json(LoginResponse {
            token: auth::create_jwt(user.user_id, user.name),
        })),
        Err(err) => {
            if err.eq(&Error::NotFound) {
                Err(NotFound(err.to_string()))
            } else {
                panic!("{}", err.to_string())
            }
        }
    };
}

struct UserFromToken(User);
impl<'a, 'r> FromRequest<'a, 'r> for UserFromToken {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        match token {
            Some(token) => match extract_jwt(token.to_string()) {
                Ok(claims) => {
                    let controller = UsersController();
                    match controller.get_user(claims.uid) {
                        Ok(user) => Outcome::Success(UserFromToken(user)),
                        // no user
                        Err(e) => {
                            if e.eq(&Error::NotFound) {
                                // Outcome::Failure((Status::Unauthorized, Unauthorized::<String>(Some("user from token does not exist".to_string()))))
                                Outcome::Failure((
                                    Status::Unauthorized,
                                    "user from token does not exist".to_string(),
                                ))
                            } else {
                                panic!("{}", e.to_string())
                            }
                        }
                    }
                }
                // invalid token
                // Err(e) => Outcome::Failure((Status::Unauthorized, "invalid token".to_string()))
                Err(e) => panic!("{}", e.to_string()),
            },
            // token does not exist
            None => Outcome::Failure((Status::Unauthorized, "token does not exist".to_string())), // None => Outcome::Failure((Status::Unauthorized))
        }
    }
}

#[get("/some_private_endpoint")]
fn private_endpoint(user: UserFromToken) -> String {
    return format!("{}, {}", user.0.user_id.to_string(), user.0.name);
}

// #[post("/registration", format="json", data="<name>")]
// pub fn login(data: Json<Login>) -> Result<LoginResponse, String> {
//     let controller : UsersController = UsersController();
//     return Json(controller.get_user(data.name));
// }

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let controller: UsersController = UsersController();
    return Json(controller.get_users());
}

#[post("/users", format = "json", data = "<data>")]
fn post_users(data: Json<NewUser>) -> Json<User> {
    let controller: UsersController = UsersController();
    return Json(controller.create_user(data.into_inner()));
}

#[get("/users/<user_id>")]
fn get_user(user_id: i32) -> Result<Json<User>, NotFound<String>> {
    let controller: UsersController = UsersController();
    return match controller.get_user(user_id) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            if err.eq(&Error::NotFound) {
                Err(NotFound(err.to_string()))
            } else {
                panic!("{}", err.to_string())
            }
        }
    };
}

#[put("/users/<user_id>", format = "json", data = "<data>")]
fn put_user(user_id: i32, data: Json<UpdatedUser>) -> Result<Json<User>, NotFound<String>> {
    let controller: UsersController = UsersController();
    return match controller.update_user(user_id, data.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            if err.eq(&Error::NotFound) {
                Err(NotFound(err.to_string()))
            } else {
                panic!("{}", err.to_string())
            }
        }
    };
}

#[delete("/users/<user_id>")]
fn delete_user(user_id: i32) -> Result<Status, NotFound<String>> {
    let controller: UsersController = UsersController();
    return match controller.delete_user(user_id) {
        Ok(_res) => {
            if _res == 0 {
                Err(NotFound("Not found".to_string()))
            } else {
                Ok(Status::Ok)
            }
        }
        Err(err) => panic!("{}", err.to_string()),
    };
}

fn main() {
    let uid = 3331;
    let name = "maksim";
    //println!("{}",create_jwt(uid, name.to_string()));
    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                login,
                get_users,
                post_users,
                get_user,
                put_user,
                delete_user,
                private_endpoint
            ],
        )
        .launch();
}
