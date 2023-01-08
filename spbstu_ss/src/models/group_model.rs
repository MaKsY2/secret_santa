use diesel::{Queryable, Insertable, AsChangeset};

use serde_derive::{Serialize, Deserialize};

use crate::schema::groups;

#[derive(Queryable, Serialize)]
#[diesel(table_name = groups)]
pub struct Group {
    pub group_id: i32,
    pub name: String,
    pub status: String
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    pub name: String
}
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = groups)]
pub struct UpdatedGroup {
    pub name: String
}

#[derive(AsChangeset)]
#[diesel(table_name = groups)]
pub struct CloseGroup {
    pub status: String
}
