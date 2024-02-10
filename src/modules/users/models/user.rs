use crate::schema::user;
use diesel::internal::derives::multiconnection::chrono;
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct New<'a> {
    pub id: Option<Uuid>,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub mail: &'a str,
    pub tel: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entity {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub tel: String,
    pub blocked_at: Option<chrono::NaiveDateTime>,
    pub blocked_by: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
