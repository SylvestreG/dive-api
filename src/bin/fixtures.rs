use std::collections::HashMap;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dive_api::error::DiveErr;
use dive_api::modules::core::fixtures::Trait;
use dive_api::modules::users::services;
#[must_use]
pub fn fixtures(pool: &Pool<ConnectionManager<PgConnection>>) -> HashMap<&str, Box<dyn Trait>> {
    let user_service = services::user::Service::new(pool.clone());
    let mut hashmap: HashMap<&str, Box<dyn Trait>> = HashMap::new();
    hashmap.insert("user", user_service.fixtures());
    hashmap
}

pub fn main() -> Result<(), DiveErr> {
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://dive-admin:secret@localhost:4432/dive-db");
    let pool = Pool::builder().build(manager)?;

    let drop_order = vec![
      "user"
    ];

    let load_order = vec![
        "user"
    ];

    let mut fixtures = fixtures(&pool);

    for order in drop_order {
        match fixtures.get_mut(order) {
            None => panic!("entry not present in fixtures"),
            Some(fixture) => fixture.delete()?
        }
    }


    for order in load_order {
        match fixtures.get_mut(order) {
            None => panic!("entry not present in fixtures"),
            Some(fixture) => fixture.load()?
        }
    }

    Ok(())
}
