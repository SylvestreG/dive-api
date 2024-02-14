use crate::schema::user_level;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = user_level)]
pub struct New {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub level_id: Uuid,
}
