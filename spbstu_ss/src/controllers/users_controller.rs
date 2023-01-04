use diesel::prelude::*;

use crate::models::*;
use crate::establish_connection;

pub struct UsersController();

pub trait UsersControllerTraits {
    fn get_users(&self) -> Vec<User>;
    fn create_user(&self, data: NewUser) -> User;
    fn get_user(&self, user_id: i32) -> User;
    fn update_user(&self, user_id: i32, data: UpdatedUser) -> User;
    fn delete_user(&self, user_id: i32);
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
        .expect("Error saving new user")
    }
    fn get_user(&self, _user_id: i32) -> User {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return users
            .filter(user_id.eq(_user_id))
            .first::<User>(&mut connection)
            .unwrap();
    }
    fn update_user(&self, _user_id: i32, data: UpdatedUser) -> User {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        return diesel::update(users.filter(user_id.eq(_user_id)))
            .set(&data)
            .get_result::<User>(&mut connection)
            .unwrap();
    }
    fn delete_user(&self, _user_id: i32) {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        diesel::delete(users.filter(user_id.eq(_user_id)))
            .execute(&mut connection)
            .expect("Error deleting user");
    }
}
