#![feature(proc_macro_hygiene, decl_macro)]

mod auth;


use diesel::result::Error;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::*;
use rocket_contrib::json::Json;
use spbstu_ss::controllers::memberships_controller::{MembershipsController, MembershipsControllerTraits};
use std::convert::Infallible;

use auth::*;
use spbstu_ss::controllers::users_controller::*;
use spbstu_ss::controllers::groups_controller::*;
use spbstu_ss::models::user_model::{NewUser, UpdatedUser, User};
use spbstu_ss::models::group_model::{Group, NewGroup, UpdatedGroup};
use spbstu_ss::models::membership_model::{Membership, NewMembership, UpdatedMembership};
use spbstu_ss::models::auth_model::{Claims, LoginRequest, LoginResponse, RegistrationRequest, RegistrationResponse};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello? {} year old named {}!", age, name)
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let controller: UsersController = UsersController();
    let user_check = conntroller.check_password(data.name, data.password);
    if !user_check {
        return Err(Status::Unauthorized);
    }
    let user_res = controller.get_user_by_name(name.into_inner().name.clone());
    return match user_res {
        Ok(user) => Ok(Json(LoginResponse {
            token: auth::create_jwt(user.user_id, user.name),
            user: User
        })),
        Err(err) => {
            if err.eq(&Error::NotFound) {
                return Err(Status::Unauthorized);
            }
            panic!("{}", err.to_string())
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

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let controller: UsersController = UsersController();
    return Json(controller.get_users());
}

#[post("/users", format = "json", data = "<data>")]
fn post_users(data: Json<NewUser>) -> Result<Json<User>, Status> {
    let controller = UsersController();
    match controller.get_user_by_name(data.into_inner().name) {
        Ok(_t) => return Err(Status::Conflict),
        Err(e) => {
            if err.eq(&Error::NotFound) {
                return Ok(Json(controller.create_user(data.into_inner())));
            }
            panic!("{}", err.to_string())
        }
    }
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
fn put_user(user_id: i32, data: Json<UpdatedUser>, user: UserFromToken) -> Result<Json<User>, Status> {
    let controller: UsersController = UsersController();
    if user.0.user_id != user_id {
        return Err(Status::Unauthorized);
    }
    return match controller.update_user(user_id, data.into_inner()) {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            if err.eq(&Error::NotFound) {
                Err(Status::NotFound)
            } else {
                panic!("{}", err.to_string())
            }
        }
    };
}

#[delete("/users/<user_id>")]
fn delete_user(user_id: i32, user: UserFromToken) -> Result<Status, Status> {
    if user.0.user_id != user_id {
        return Err(Status::Unauthorized);
    }
    let controller: UsersController = UsersController();
    return match controller.delete_user(user_id) {
        Ok(_res) => {
            if _res == 0 {
                Err(Status::NotFound)
            } else {
                Ok(Status::Ok)
            }
        },
        Err(err) => panic!("{}", err.to_string())
    };
}

#[get("/memberships?<group_id>&<user_id>")]
fn get_memberships(group_id: Option<i32>, user_id: Option<i32>) -> Json<Vec<Membership>> {
    let controller : MembershipsController = MembershipsController();
    return Json(controller.get_memberships(group_id, user_id));
}

#[put("/memberships?<group_id>&<user_id>", format = "json", data = "<data>")]
fn put_membership(group_id: i32, user_id: i32, data: Json<UpdatedMembership>, user: UserFromToken)
        -> Result<Json<Membership>, Status> {
    let controller : MembershipsController = MembershipsController();
    let my_membership = controller.get_membership(group_id, user.0.user_id);
    match my_membership {
        Err(e) => if err.eq(&Error::NotFound)
        { return Err(Status::Unauthorized) } else { panic!("{}", err.to_string()) },
        Ok(m) => if m.role != "admin" { return Err(Status::Unauthorized); }
    }
    return match controller.update_membership(group_id, user_id, data.into_inner()) {
        Ok(membership) => Ok(Json(membership)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(Status::NotFound) } else { panic!("{}", err.to_string()) }
    }
}

#[post("/memberships", format = "json", data = "<data>")]
fn post_membership(data: Json<NewMembership>, user: UserFromToken) -> Result<Json<Membership>, Status> {
    if user.0.user_id != data.into_inner().user_id {
        return Err(Status::Unauthorized);
    }
    let controller : MembershipsController = MembershipsController();
    return Ok(Json(controller.create_membership(data.into_inner())));
}

#[delete("/memberships?<group_id>&<user_id>")]
fn delete_membership(group_id: i32, user_id: i32, user: UserFromToken) -> Result<Status, Status> {
    let controller: MembershipsController = MembershipsController();
    let my_membership = controller.get_membership(group_id, user.0.user_id);
    match my_membership {
        Ok(m) => {
            if user.0.user_id != data.into_inner().user_id && m.role != "admin" {
                return Err(Status::Unauthorized);
            }
        },
        Err(e) => return Err(Status::Unauthorized)
    }
    if user.0.user_id != data.into_inner().user_id {
        return Err(Status::Unauthorized);
    }
    return match controller.delete_membership(group_id, user_id) {
        Ok(_res) => if _res == 0 { Err(Status::NotFound) } else { Ok(Status::Ok) },
        Err(err) => panic!("{}", err.to_string())
    };
}

#[get("/groups")]
fn get_groups() -> Json<Vec<Group>> {
    let controller : GroupsController = GroupsController();
    return Json(controller.get_groups());
}
#[post("/groups", format = "json", data = "<data>")]
fn post_groups(data: Json<NewGroup>) -> Json<Group> {
    let controller : GroupsController = GroupsController();
    return Json(controller.create_group(data.into_inner()));
}
#[get("/groups/<group_id>")]
fn get_group(group_id: i32) -> Result<Json<Group>, NotFound<String>> {
    let controller : GroupsController = GroupsController();
    return match controller.get_group(group_id) {
        Ok(group) => Ok(Json(group)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(NotFound(err.to_string())) } else { panic!("{}", err.to_string()) }
    };
}
#[put("/groups/<group_id>", format = "json", data = "<data>")]
fn put_group(group_id: i32, data: Json<UpdatedGroup>) -> Result<Json<Group>, NotFound<String>> {
    let controller : GroupsController = GroupsController();
    return match controller.update_group(group_id, data.into_inner()) {
        Ok(group) => Ok(Json(group)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(NotFound(err.to_string())) } else { panic!("{}", err.to_string()) }
    }
}

#[delete("/groups/<group_id>")]
fn delete_group(group_id: i32) -> Result<Status, Status> {
    let controller : GroupsController = GroupsController();
    return match controller.delete_group(group_id) {
        Ok(_res) => if _res == 0 {Err(Status::NotFound)} else {Ok(Status::Ok)},
        Err(err) => panic!("{}", err.to_string())
    };
}

fn main() {
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

//TODO допилить delete_groups
