create table "user"
(
    "id"         uuid      not null primary key default uuid_generate_v4(),
    "firstname"  varchar   not null,
    "lastname"   varchar   not null,
    "mail"       varchar   not null,
    "tel"        varchar   not null,

    "blocked_at" timestamp,
    "blocked_by" uuid,

    "created_at" timestamp not null             default now(),
    "updated_at" timestamp not null             default now(),

    unique (mail)
);

select diesel_manage_updated_at('user');