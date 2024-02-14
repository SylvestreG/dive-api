use crate::error::DiveErr;
use crate::modules::users::models::level::{Entity, New};
use diesel::r2d2::ConnectionManager;
use diesel::SelectableHelper;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::{PgConnection, QueryDsl};
use r2d2::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Repository {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl Repository {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> Self {
        Repository { conn }
    }

    pub fn new_level(&mut self, new_level: &New) -> Result<(), DiveErr> {
        use crate::schema::level;

        let entity = diesel::insert_into(level::table)
            .values(new_level)
            .returning(Entity::as_returning())
            .get_result(&mut self.conn.get()?)?;

        log::debug!("new {} level inserted", entity.id);

        Ok(())
    }

    pub fn get(&self, uuid: Uuid) -> Result<Entity, DiveErr> {
        use crate::schema::level;

        Ok(level::table
            .select(Entity::as_select())
            .filter(level::id.eq(uuid))
            .first(&mut self.conn.get()?)?)
    }

    pub fn all(&self) -> Result<Vec<Entity>, DiveErr> {
        use crate::schema::level;

        Ok(level::table
            .select(Entity::as_select())
            .load(&mut self.conn.get()?)?)
    }

    pub fn drop_all(&mut self) -> Result<(), DiveErr> {
        use crate::schema::level;

        let nb_rows = diesel::delete(level::table).execute(&mut self.conn.get()?)?;

        log::debug!("{} levels deleted", nb_rows);

        Ok(())
    }
}
