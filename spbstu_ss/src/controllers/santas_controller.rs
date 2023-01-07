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
    fn generate_santas(&self, group_id: i32) -> Result<Vec<Santa>, Error>;
}


impl SantasControllerTraits for SantasController {
    fn get_santas(&self, _group_id: i32) -> Result<Vec<Santa>, Error> {
        use crate::schema::santas::dsl::*;
        let mut connection = establish_connection();
        return santas
            .filter(group_id.eq(_group_id))
            .load::<Santa>(&mut connection);
    }
    fn generate_santas(&self, group_id: i32) -> Result<Vec<Santa>, Error> {
        let memberships_ctrl = MembershipsController();
        let mut memberships = memberships_ctrl.get_memberships(group_id);
        if memberships.len() == 0 {
            return Ok(vec![]);
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
        use crate::schema::santas;
        let mut connection = establish_connection();
        return diesel::insert_into(santas::table)
            .values(&santas)
            .get_results(&mut connection);
    }
}
