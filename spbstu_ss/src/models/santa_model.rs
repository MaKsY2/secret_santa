use diesel::{Queryable, Insertable};

use serde_derive::{Deserialize, Serialize};

use crate::schema::santas;

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = santas)]
pub struct Santa {
    pub(crate) group_id: i32,
    pub(crate) santa_user_id: i32,
    pub(crate) receiver_user_id: i32
}
