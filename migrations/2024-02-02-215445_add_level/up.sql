create table "level"
(
    "id"         uuid      not null primary key default uuid_generate_v4(),
    "level_name" varchar   not null,

    "created_at" timestamp not null             default now(),
    "updated_at" timestamp not null             default now(),

    unique (level_name)
);

-- fk + constraint
create table "level_dep_link"
(
    "id"              uuid      not null primary key default uuid_generate_v4(),
    "level_id"        uuid      not null,
    "level_needed_id" uuid      not null,

    "created_at"      timestamp not null             default now(),
    "updated_at"      timestamp not null             default now(),

    unique(level_id, level_needed_id),

    constraint fk_level
        foreign key (level_id)
            references level (id)
            on update cascade
            on delete cascade,
    constraint fk_level_needed
        foreign key (level_needed_id)
            references level (id)
            on update cascade
            on delete cascade
);

-- fk + constraint
create table "user_level"
(
    "id"                    uuid      not null primary key default uuid_generate_v4(),
    "user_id"               uuid      not null,
    "level_id"              uuid      not null,

    "theory_validated_at"   timestamp,
    "practice_validated_at" timestamp,

    "created_at"            timestamp not null             default now(),
    "updated_at"            timestamp not null             default now(),

    unique (user_id, level_id),

    constraint fk_user
        foreign key (user_id)
            references "user" (id)
            on update cascade
            on delete cascade,
    constraint fk_level
        foreign key (level_id)
            references level (id)
            on update cascade
            on delete cascade
);

select diesel_manage_updated_at('level');
select diesel_manage_updated_at('level_dep_link');
select diesel_manage_updated_at('user_level');