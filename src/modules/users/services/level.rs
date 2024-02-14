use crate::error::DiveErr;
use crate::modules::users::{fixtures, models, repositories};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use uuid::Uuid;

pub struct Service {
    repository: repositories::level::Repository,
}

impl Service {
    #[must_use]
    pub fn new(con: Pool<ConnectionManager<PgConnection>>) -> Self {
        Service {
            repository: repositories::level::Repository::new(con),
        }
    }

    #[must_use]
    pub fn fixtures(&self) -> Box<fixtures::level::Fixtures> {
        Box::new(fixtures::level::Fixtures::new(self.repository.clone()))
    }

    pub fn find_by_uuid(&self, uuid: Uuid) -> Result<models::level::Entity, DiveErr> {
        // TODO ACL

        self.repository.get(uuid)
    }

    pub fn list_all_levels(&self) -> Result<Vec<models::level::Entity>, DiveErr> {
        // TODO ACL

        self.repository.all()
    }

    #[must_use]
    pub fn user_by_role(&self) -> Vec<models::level::Entity> {
        vec![]
    }
}
