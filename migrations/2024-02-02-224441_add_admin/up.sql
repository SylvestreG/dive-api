create table "admin"
(
    "id"         uuid      not null primary key default uuid_generate_v4(),
    "user_id"    uuid      not null,

    "granted_by" uuid,

    "created_at" timestamp not null             default now(),
    "updated_at" timestamp not null             default now(),

    unique (id, user_id),
    constraint fk_user
        foreign key (user_id)
            references "user" (id)
            on update cascade
            on delete cascade
);

select diesel_manage_updated_at('admin');
