use diesel::prelude::*;
use diesel::result::Error;

use crate::establish_connection;
use crate::models::user_model::*;

pub struct UsersController();

pub trait UsersControllerTraits {
    fn get_users(&self) -> Vec<User>;
    fn create_user(&self, data: NewUser) -> User;
    fn get_user(&self, user_id: i32) -> Result<User, Error>;
    fn get_user_by_name(&self, user_name: String) -> Result<User, Error>;
    fn update_user(&self, user_id: i32, data: UpdatedUser) -> Result<User, Error>;
    fn delete_user(&self, user_id: i32) -> Result<usize, Error>;
}

impl UsersControllerTraits for UsersController {
    fn get_users(&self) -> Vec<User> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return users
            .load::<User>(&mut connection)
            .expect("Error loading users");
    }
    fn create_user(&self, data: NewUser) -> User {
        use crate::schema::users;
        let mut connection = establish_connection();
        return diesel::insert_into(users::table)
            .values(&data)
            .get_result::<User>(&mut connection)
            .expect("Error saving new user");
    }
    fn get_user(&self, _user_id: i32) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return users
            .filter(user_id.eq(_user_id))
            .first::<User>(&mut connection);
    }
    fn get_user_by_name(&self, user_name: String) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return users
            .filter(name.eq(user_name))
            .first::<User>(&mut connection);
    }
    fn update_user(&self, _user_id: i32, data: UpdatedUser) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return diesel::update(users.filter(user_id.eq(_user_id)))
            .set(&data)
            .get_result::<User>(&mut connection);
    }
    fn delete_user(&self, _user_id: i32) -> Result<usize, Error> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return diesel::delete(users.filter(user_id.eq(_user_id))).execute(&mut connection);
    }
}
