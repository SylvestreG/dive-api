use crate::schema::level;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = level)]
pub struct New<'a> {
    pub id: Option<Uuid>,
    pub level_name: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::level)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entity {
    pub id: Uuid,
    pub level_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
