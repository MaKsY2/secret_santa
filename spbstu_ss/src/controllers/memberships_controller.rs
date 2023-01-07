use diesel::prelude::*;
use diesel::result::Error;

use crate::models::membership_model::*;
use crate::establish_connection;

pub struct MembershipsController();

pub trait MembershipsControllerTraits {
    fn get_memberships(&self, group_id: Option<i32>, user_id: Option<i32>) -> Vec<Membership>;
    fn create_membership(&self, data: NewMembership) -> Membership;
    fn get_membership(&self, group_id: i32, user_id: i32) -> Result<Membership, Error>;
    fn update_membership(&self, group_id:i32, user_id: i32, data: UpdatedMembership) -> Result<Membership, Error>;
    fn delete_membership(&self, group_id:i32, user_id: i32) -> Result<usize, Error>;
}

impl MembershipsControllerTraits for MembershipsController {
    fn get_memberships(&self, _group_id: Option<i32>, _user_id: Option<i32>) -> Vec<Membership> {
        use crate::schema::memberships::dsl::*;
        let mut connection = establish_connection();
        let mut query = memberships;
        if _group_id.is_some() {
            query = query.filter(group_id.eq(_group_id));
        }
        if _user_id.is_some() {
            query = query.filter(user_id.eq(_user_id));
        }
        return match query
            .load::<Membership>(&mut connection) {
            Ok(res) => res,
            Err(_res) => Vec::new()
        };
    }
    fn create_membership(&self, data: NewMembership) -> Membership {
        use crate::schema::memberships;
        let mut connection = establish_connection();
        return diesel::insert_into(memberships::table)
            .values(&data)
            .get_result::<Membership>(&mut connection)
            .expect("Error saving new membership")
    }
    fn get_membership(&self, _group_id: i32, _user_id: i32) -> Result<Membership, Error> {
        use crate::schema::memberships::dsl::*;
        let mut connection = establish_connection();
        return memberships
            .filter(group_id.eq(_group_id).and(user_id.eq(_user_id)))
            .first::<Membership>(&mut connection);
    }
    fn update_membership(&self, _group_id:i32, _user_id: i32, data: UpdatedMembership) -> Result<Membership, Error> {
        use crate::schema::memberships::dsl::*;
        let mut connection = establish_connection();
        return diesel::update(memberships
                .filter(group_id.eq(_group_id).and(user_id.eq(_user_id))))
            .set(&data)
            .get_result::<Membership>(&mut connection);
    }
    fn delete_membership(&self, _group_id: i32, _user_id: i32) -> Result<usize, Error> {
        use crate::schema::memberships::dsl::*;
        let mut connection = establish_connection();
        return diesel::delete(memberships
                .filter(group_id.eq(_group_id).and(user_id.eq(_user_id))))
            .execute(&mut connection);
    }
}