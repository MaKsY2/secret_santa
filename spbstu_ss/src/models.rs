use diesel::{Queryable, Insertable, AsChangeset};

use serde_derive::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdatedUser {
    pub name: String
}

#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub name: String,
    pub uid: i32,
}
