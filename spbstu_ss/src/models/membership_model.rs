use diesel::{Queryable, Insertable, AsChangeset};

use serde_derive::{Serialize, Deserialize};

use crate::myschema::memberships;

#[derive(Queryable, Serialize)]
#[diesel(table_name = memberships)]
pub struct Membership {
    pub group_id: i32,
    pub user_id: i32,
    pub role: String
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = memberships)]
pub struct NewMembership {
    pub group_id: i32,
    pub user_id: i32
}
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = memberships)]
pub struct UpdatedMembership {
    pub role: String
}
