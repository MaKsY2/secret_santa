use diesel::prelude::*;
use diesel::result::Error;

use crate::establish_connection;
use crate::models::user_model::*;

pub struct UsersController();

pub trait UsersControllerTraits {
    fn get_users(&self) -> Vec<User>;
    fn create_user(&self, data: NewUser) -> User;
    fn get_user(&self, user_id: i32) -> Result<User, Error>;
    fn get_user_by_name(&self, user_name: &String) -> Result<User, Error>;
    fn update_user(&self, user_id: i32, data: UpdatedUser) -> Result<User, Error>;
    fn delete_user(&self, user_id: i32) -> Result<usize, Error>;
    fn check_password(&self, _name: &String, _password: &String) -> bool;
}

impl UsersControllerTraits for UsersController {
    fn get_users(&self) -> Vec<User> {
        use crate::myschema::users_wo_passwords::dsl::*;
        let mut connection = establish_connection();
        return users_wo_passwords
            .load::<User>(&mut connection)
            .expect("Error loading users");
    }
    fn create_user(&self, data: NewUser) -> User {
        use crate::myschema::users;
        let mut connection = establish_connection();
        let uwp = diesel::insert_into(users::table)
            .values(&data)
            .get_result::<UserWithPassword>(&mut connection)
            .expect("Error saving new user");
        return User {
            user_id: uwp.user_id,
            name: uwp.name
        }
    }
    fn get_user(&self, _user_id: i32) -> Result<User, Error> {
        use crate::myschema::users_wo_passwords::dsl::*;
        let mut connection = establish_connection();
        return users_wo_passwords
            .filter(user_id.eq(_user_id))
            .first::<User>(&mut connection);
    }
    fn get_user_by_name(&self, user_name: &String) -> Result<User, Error> {
        use crate::myschema::users_wo_passwords::dsl::*;
        let mut connection = establish_connection();
        return users_wo_passwords
            .filter(name.eq(user_name))
            .first::<User>(&mut connection);
    }
    fn update_user(&self, _user_id: i32, data: UpdatedUser) -> Result<User, Error> {
        use crate::myschema::users::dsl::*;
        let mut connection = establish_connection();
        let uwp = match diesel::update(users.filter(user_id.eq(_user_id)))
            .set(&data)
            .get_result::<UserWithPassword>(&mut connection) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };
        return Ok(User {
            user_id: uwp.user_id,
            name: uwp.name
        })
    }
    fn delete_user(&self, _user_id: i32) -> Result<usize, Error> {
        use crate::myschema::users::dsl::*;
        let mut connection = establish_connection();
        return diesel::delete(users.filter(user_id.eq(_user_id))).execute(&mut connection);
    }

    fn check_password(&self, _name: &String, _password: &String) -> bool {
        use crate::myschema::users::dsl::*;
        let mut connection = establish_connection();
        return users
            .filter(name.eq(_name))
            .filter(password.eq(_password))
            .first::<UserWithPassword>(&mut connection).is_ok();
    }
}
