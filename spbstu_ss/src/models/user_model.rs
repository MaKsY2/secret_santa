use diesel::{Queryable, Insertable, AsChangeset};

use serde_derive::{Serialize, Deserialize};

use crate::myschema::users;

#[derive(Queryable, Serialize)]
#[diesel(table_name = users_wo_passwords)]
pub struct User {
    pub user_id: i32,
    pub name: String
}
#[derive(Queryable, Serialize)]
#[diesel(table_name = users_wo_passwords)]
pub struct UserWithPassword {
    pub user_id: i32,
    pub name: String,
    pub password: String
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub password: String
}
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdatedUser {
    pub name: String
}
