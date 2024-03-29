use crate::error::DiveErr;
use crate::modules::users::{fixtures, models, repositories};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use uuid::Uuid;

pub struct Service {
    repository: repositories::user::Repository,
}

impl Service {
    #[must_use]
    pub fn new(con: Pool<ConnectionManager<PgConnection>>) -> Self {
        Service {
            repository: repositories::user::Repository::new(con),
        }
    }

    #[must_use]
    pub fn fixtures(&self) -> Box<fixtures::user::Fixtures> {
        Box::new(fixtures::user::Fixtures::new(self.repository.clone()))
    }

    pub fn find_by_uuid(&self, uuid: Uuid) -> Result<models::user::Entity, DiveErr> {
        // TODO ACL

        self.repository.get(uuid)
    }

    pub fn list_all_user(&self) -> Result<Vec<models::user::Entity>, DiveErr> {
        // TODO ACL

        self.repository.all()
    }

    #[must_use]
    pub fn user_by_role(&self) -> Vec<models::user::Entity> {
        vec![]
    }
}
