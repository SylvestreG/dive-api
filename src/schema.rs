// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "environment"))]
    pub struct Environment;
}

diesel::table! {
    ability (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        level_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    level_ability (id) {
        id -> Uuid,
        level_id -> Uuid,
        ability_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    level_dep_link (id) {
        id -> Uuid,
        level_id -> Uuid,
        level_needed_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        mail -> Varchar,
        tel -> Varchar,
        blocked_at -> Nullable<Timestamp>,
        blocked_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_level (id) {
        id -> Uuid,
        user_id -> Uuid,
        level_id -> Uuid,
        theory_validated_at -> Nullable<Timestamp>,
        practice_validated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Environment;

    user_level_ability (id) {
        id -> Uuid,
        level_ability_id -> Uuid,
        user_id -> Uuid,
        environment -> Nullable<Environment>,
        validated_at -> Nullable<Timestamp>,
        validated_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(level_ability -> ability (ability_id));
diesel::joinable!(level_ability -> level (level_id));
diesel::joinable!(user_level -> level (level_id));
diesel::joinable!(user_level -> user (user_id));
diesel::joinable!(user_level_ability -> level_ability (level_ability_id));
diesel::joinable!(user_level_ability -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ability,
    level,
    level_ability,
    level_dep_link,
    user,
    user_level,
    user_level_ability,
);
