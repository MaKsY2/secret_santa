use diesel::prelude::*;
use diesel::result::Error;
use crate::controllers::memberships_controller::{MembershipsController, MembershipsControllerTraits};

use crate::models::santa_model::*;
use crate::establish_connection;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct SantasController();

pub trait SantasControllerTraits {
    fn get_santas(&self, group_id: i32) -> Result<Vec<Santa>, Error>;
    fn generate_santas(&self, group_id: i32) -> Result<usize, Error>;
    fn get_santa(&self, _group_id: i32, _user_id: i32) -> Result<Santa, Error>;
}


impl SantasControllerTraits for SantasController {
    fn get_santas(&self, _group_id: i32) -> Result<Vec<Santa>, Error> {
        use crate::myschema::santas::dsl::*;
        let mut connection = establish_connection();
        return santas
            .filter(group_id.eq(_group_id))
            .load::<Santa>(&mut connection);
    }
    fn get_santa(&self, _group_id: i32, _user_id: i32) -> Result<Santa, Error>{
        use crate::myschema::santas::dsl::*;
        let mut connection = establish_connection();
        return santas
            .filter(group_id.eq(_group_id).and(santa_user_id.eq(_user_id)))
            .first::<Santa>(&mut connection);
    }
    fn generate_santas(&self, group_id: i32) -> Result<usize, Error> {
        let memberships_ctrl = MembershipsController();
        let mut memberships = memberships_ctrl.get_memberships(Some(group_id), None);
        if memberships.len() == 0 {
            return Ok(0);
        }

        memberships.shuffle(&mut thread_rng());
        let mut santas: Vec<Santa> = vec![];
        for i in 0..memberships.len() - 1 {
            santas.push(
                Santa{group_id: group_id,
                    santa_user_id: memberships.get(i).unwrap().user_id,
                    receiver_user_id: memberships.get(i + 1).unwrap().user_id}
            )
        }
        santas.push(
                Santa{group_id: group_id,
                    santa_user_id: memberships.last().unwrap().user_id,
                    receiver_user_id: memberships.first().unwrap().user_id}
        );
        use crate::myschema::santas;
        let mut connection = establish_connection();
        return diesel::insert_into(santas::table)
            .values(&santas)
            .execute(&mut connection);
    }
}
