#![feature(proc_macro_hygiene, decl_macro)]

mod auth;


use diesel::result::Error;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::status::{NotFound};
use rocket::*;
use rocket_contrib::json::Json;
use spbstu_ss::controllers::memberships_controller::{MembershipsController, MembershipsControllerTraits};

use auth::*;
use spbstu_ss::controllers::users_controller::*;
use spbstu_ss::controllers::groups_controller::*;
use spbstu_ss::controllers::santas_controller::{SantasController, SantasControllerTraits};
use spbstu_ss::models::user_model::{NewUser, UpdatedUser, User};
use spbstu_ss::models::group_model::{Group, NewGroup, UpdatedGroup};
use spbstu_ss::models::membership_model::{Membership, NewMembership, UpdatedMembership};
use spbstu_ss::models::santa_model::Santa;
use spbstu_ss::models::auth_model::{LoginRequest, LoginResponse};


#[post("/login", format = "json", data = "<data_raw>")]
pub fn login(data_raw: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let controller: UsersController = UsersController();
    let data = data_raw.into_inner();
    let user_check = controller.check_password(&data.name, &data.password);
    if !user_check {
        return Err(Status::Unauthorized);
    }
    let user_res = controller.get_user_by_name(&data.name);
    return match user_res {
        Ok(user) => Ok(Json(LoginResponse {
            token: auth::create_jwt(user.user_id, &user.name),
            user: user
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

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let controller: UsersController = UsersController();
    return Json(controller.get_users());
}

#[post("/users", format = "json", data = "<data_raw>")]
fn post_users(data_raw: Json<NewUser>) -> Result<Json<User>, Status> {
    let controller = UsersController();
    let data = data_raw.into_inner();

    match controller.get_user_by_name(&data.name) {
        Ok(_t) => return Err(Status::Conflict),
        Err(e) => {
            if e.eq(&Error::NotFound) {
                return Ok(Json(controller.create_user(data)));
            }
            panic!("{}", e.to_string())
        }
    }
}

#[get("/users/<user_id>")]
fn get_user(user_id: i32) -> Result<Json<User>, Status> {
    let controller: UsersController = UsersController();
    return match controller.get_user(user_id) {
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
        Err(e) => if e.eq(&Error::NotFound)
        { return Err(Status::Unauthorized) } else { panic!("{}", e.to_string()) },
        Ok(m) => if m.role != "admin" { return Err(Status::Unauthorized); }
    }
    if user.0.user_id == user_id {
        let memes = controller.get_memberships(Some(group_id), None);
        let mut f = false;
        for meme in memes {
            if meme.user_id != user.0.user_id && meme.role == "admin" {
                f = true;
            }
        }
        if !f {return Err(Status::Conflict);}
    }
    return match controller.update_membership(group_id, user_id, data.into_inner()) {
        Ok(membership) => Ok(Json(membership)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(Status::NotFound) } else { panic!("{}", err.to_string()) }
    }
}

#[post("/memberships", format = "json", data = "<data_raw>")]
fn post_membership(data_raw: Json<NewMembership>, user: UserFromToken) -> Result<Json<Membership>, Status> {
    let data = data_raw.into_inner();
    let groups_ctrl = GroupsController();
    let group = match groups_ctrl.get_group(data.group_id) {
        Ok(g) => g,
        Err(e) => if e.eq(&Error::NotFound)
        { return Err(Status::NotFound) } else { panic!("{}", e.to_string()) }
    };
    if group.status == "closed" {
        return Err(Status::Conflict);
    }
    if user.0.user_id != data.user_id {
        return Err(Status::Unauthorized);
    }
    let controller : MembershipsController = MembershipsController();
    return Ok(Json(controller.create_membership(data)));
}

#[delete("/memberships?<group_id>&<user_id>")]
fn delete_membership(group_id: i32, user_id: i32, user: UserFromToken) -> Result<Status, Status> {
    let groups_ctrl = GroupsController();
    let group = match groups_ctrl.get_group(group_id) {
        Ok(g) => g,
        Err(e) => if e.eq(&Error::NotFound)
        { return Err(Status::NotFound) } else { panic!("{}", e.to_string()) }
    };
    if group.status == "closed" {
        return Err(Status::Conflict);
    }
    let controller: MembershipsController = MembershipsController();
    let my_membership = controller.get_membership(group_id, user.0.user_id);
    match my_membership {
        Ok(m) => {
            if user.0.user_id != user_id && m.role != "admin" {
                return Err(Status::Unauthorized);
            }
        },
        Err(_e) => return Err(Status::Unauthorized)
    }
    if user.0.user_id != user_id {
        return Err(Status::Unauthorized);
    }
    let memes = controller.get_memberships(Some(group_id), None);
    let mut f = false;
    for meme in memes {
        if meme.user_id != user.0.user_id && meme.role == "admin" {
            f = true;
        }
    }
    if !f {return Err(Status::Conflict);}
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
fn post_groups(data: Json<NewGroup>, user: UserFromToken) -> Json<Group> {
    let controller : GroupsController = GroupsController();
    let memberships_controller = MembershipsController();
    let group = controller.create_group(data.into_inner());
    let _meme = memberships_controller.create_membership(NewMembership{
        user_id: user.0.user_id,
        group_id: group.group_id
    });
    let _upd = memberships_controller
        .update_membership(group.group_id, user.0.user_id, UpdatedMembership{
            role: "admin".to_string()
        });
    return Json(group);
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
fn put_group(group_id: i32, data: Json<UpdatedGroup>, user: UserFromToken) -> Result<Json<Group>, Status> {
    let memberships_controller = MembershipsController();
    let my_membership = memberships_controller.get_membership(group_id, user.0.user_id);
    match my_membership {
        Err(e) => if e.eq(&Error::NotFound)
        { return Err(Status::Unauthorized) } else { panic!("{}", e.to_string()) },
        Ok(m) => if m.role != "admin" { return Err(Status::Unauthorized); }
    }
    let memes = memberships_controller.get_memberships(Some(group_id), None);
    let mut f = false;
    for meme in memes {
        if meme.user_id != user.0.user_id && meme.role == "admin" {
            f = true;
        }
    }
    if !f {return Err(Status::Conflict);}
    let controller : GroupsController = GroupsController();
    return match controller.update_group(group_id, data.into_inner()) {
        Ok(group) => Ok(Json(group)),
        Err(err) => if err.eq(&Error::NotFound)
        { Err(Status::Unauthorized) } else { panic!("{}", err.to_string()) }
    }
}

#[delete("/groups/<group_id>")]
fn delete_group(group_id: i32, user: UserFromToken) -> Result<Status, Status> {
    let memberships_controller = MembershipsController();
    let my_membership = memberships_controller.get_membership(group_id, user.0.user_id);
    match my_membership {
        Err(e) => if e.eq(&Error::NotFound)
        { return Err(Status::Unauthorized) } else { panic!("{}", e.to_string()) },
        Ok(m) => if m.role != "admin" { return Err(Status::Unauthorized); }
    }
    let controller : GroupsController = GroupsController();
    return match controller.delete_group(group_id) {
        Ok(_res) => if _res == 0 {Err(Status::NotFound)} else {Ok(Status::Ok)},
        Err(err) => panic!("{}", err.to_string())
    };
}

#[post("/santas?<group_id>")]
fn post_santas(group_id: i32, user: UserFromToken) -> Result<Status, Status> {
    let groups_controller = GroupsController();
    let _group = match groups_controller
            .get_group(group_id) {
        Ok(g) => g,
        Err(_e) => return Err(Status::NotFound)
    };
    let memberships_controller = MembershipsController();
    let meme = match memberships_controller
            .get_membership(group_id, user.0.user_id) {
        Ok(m) => m,
        Err(_e) => return Err(Status::Unauthorized)
    };
    if meme.role != "admin" {return Err(Status::Unauthorized);}
    let santas_controller = SantasController();
    return match santas_controller.generate_santas(group_id) {
        Ok(_s) => {
            match groups_controller.close_group(group_id) {
                Ok(_g) => Ok(Status::Ok),
                Err(e) => panic!("{}", e.to_string())
            }
        },
        Err(e) => panic!("{}", e.to_string())
    };
}

#[get("/santas?<group_id>&<user_id>")]
fn get_santa(group_id: i32, user_id: i32, user: UserFromToken) -> Result<Json<Santa>, Status> {
    if user_id != user.0.user_id {return Err(Status::Unauthorized)}
    let memberships_controller = MembershipsController();
    let _meme = match memberships_controller
        .get_membership(group_id, user_id) {
        Ok(m) => m,
        Err(_e) => return Err(Status::Unauthorized)
    };
    let santas_controller = SantasController();
    return match santas_controller.get_santa(group_id, user_id) {
        Ok(s) => Ok(Json(s)),
        Err(_e) => Err(Status::Unauthorized)
    };
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                login,
                get_users,
                post_users,
                get_user,
                put_user,
                delete_user,
                get_memberships,
                put_membership,
                delete_membership,
                get_groups,
                put_group,
                delete_group,
                post_santas,
                get_santa
            ],
        )
        .launch();
}

//TODO допилить delete_groups
