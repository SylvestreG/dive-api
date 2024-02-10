use crate::error::DiveErr;
use crate::modules::users::models::user::{New, Entity};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl};
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use r2d2::Pool;

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

        diesel::insert_into(user::table)
            .values(user)
            .returning(Entity::as_returning())
            .get_result(&mut self.conn.get()?)?;

        Ok(())
    }

    pub fn all(&self) -> Result<Vec<Entity>, DiveErr> {
        use crate::schema::user;

        Ok(user::table.select(Entity::as_select())
            .load(&mut self.conn.get()?)?)
    }

    pub fn drop_all(&mut self) -> Result<(), DiveErr> {
        use crate::schema::user;

        let nb_rows = diesel::delete(user::table)
            .execute(&mut self.conn.get()?)?;

        log::info!("{} users deleted", nb_rows);

        Ok(())
    }
}
