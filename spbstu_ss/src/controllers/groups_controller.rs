use diesel::prelude::*;
use diesel::result::Error;

use crate::models::group_model::*;
use crate::establish_connection;

pub struct GroupsController();

pub trait GroupsControllerTraits {
    fn get_groups(&self) -> Vec<Group>;
    fn create_group(&self, data: NewGroup) -> Group;
    fn get_group(&self, group_id: i32) -> Result<Group, Error>;
    fn update_group(&self, group_id:i32, data: UpdatedGroup) -> Result<Group, Error>;
    fn delete_group(&self, group_id:i32) -> Result<usize, Error>;
    fn close_group(&self, group_id: i32) -> Result<Group, Error>;
}

impl GroupsControllerTraits for GroupsController {
    fn get_groups(&self) -> Vec<Group>{
        use crate::schema::groups::dsl::*;
        let mut connection = establish_connection();
        return groups
        .load::<Group>(&mut connection)
        .expect("Error loading groups");
    }
    fn create_group(&self, data: NewGroup) -> Group{
        use crate::schema::groups;
        let mut connection = establish_connection();
        return diesel::insert_into(groups::table)
        .values(&data)
        .get_result::<Group>(&mut connection)
        .expect("Error saving new group")
    }
    fn get_group(&self, _group_id: i32) -> Result<Group, Error>{
        use crate::schema::groups::dsl::*;
        let mut connection = establish_connection();
        return groups
            .filter(group_id.eq(_group_id))
            .first::<Group>(&mut connection);
    }
    fn update_group(&self, _group_id:i32, data: UpdatedGroup) -> Result<Group, Error>{
        use crate::schema::groups::dsl::*;
        let mut connection = establish_connection();
        return diesel::update(groups.filter(group_id.eq(_group_id)))
            .set(&data)
            .get_result::<Group>(&mut connection);
    }
    fn delete_group(&self, _group_id:i32) -> Result<usize, Error>{
        use crate::schema::groups::dsl::*;
        let mut connection = establish_connection();
        return diesel::delete(groups.filter(group_id.eq(_group_id)))
            .execute(&mut connection);
    }
    fn close_group(&self, _group_id: i32) -> Result<Group, Error> {
        use crate::schema::groups::dsl::*;
        let data = CloseGroup{
            status: "closed".to_string()
        };
        let mut connection = establish_connection();
        return diesel::update(groups.filter(group_id.eq(_group_id)))
            .set(&data)
            .get_result::<Group>(&mut connection);
    }
}