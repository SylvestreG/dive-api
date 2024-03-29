use crate::error::DiveErr;
use crate::modules::users::models::user::{Entity, New};
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

    pub fn new_user(&mut self, user: &New) -> Result<(), DiveErr> {
        use crate::schema::user;

        let entity = diesel::insert_into(user::table)
            .values(user)
            .returning(Entity::as_returning())
            .get_result(&mut self.conn.get()?)?;

        log::debug!("new {} user inserted", entity.id);

        Ok(())
    }

    pub fn get(&self, uuid: Uuid) -> Result<Entity, DiveErr> {
        use crate::schema::user;

        Ok(user::table
            .select(Entity::as_select())
            .filter(user::id.eq(uuid))
            .first(&mut self.conn.get()?)?)
    }

    pub fn all(&self) -> Result<Vec<Entity>, DiveErr> {
        use crate::schema::user;

        Ok(user::table
            .select(Entity::as_select())
            .load(&mut self.conn.get()?)?)
    }

    pub fn drop_all(&mut self) -> Result<(), DiveErr> {
        use crate::schema::user;

        let nb_rows = diesel::delete(user::table).execute(&mut self.conn.get()?)?;

        log::debug!("{} users deleted", nb_rows);

        Ok(())
    }
}
