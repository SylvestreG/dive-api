create type environment AS ENUM ('pool', 'pit', 'sea');

create table "ability"
(
    "id"          uuid      not null primary key default uuid_generate_v4(),
    "name"        varchar   not null,
    "description" varchar   not null,

    unique (name),

    "created_at"  timestamp not null             default now(),
    "updated_at"  timestamp not null             default now()
);

create table "level_ability"
(
    "id"         uuid      not null primary key default uuid_generate_v4(),
    "level_id"   uuid      not null,
    "ability_id" uuid      not null,

    "created_at" timestamp not null             default now(),
    "updated_at" timestamp not null             default now(),

    unique (level_id, ability_id),

    constraint fk_level
        foreign key (level_id)
            references level (id)
            on update cascade
            on delete cascade,
    constraint fk_ability
        foreign key (ability_id)
            references ability (id)
            on update cascade
            on delete cascade
);

create table "user_level_ability"
(
    "id"               uuid      not null primary key default uuid_generate_v4(),
    "level_ability_id" uuid      not null,
    "user_id"          uuid      not null,

    "environment"      environment,
    "validated_at"     timestamp,
    "validated_by"     uuid,

    "created_at"       timestamp not null             default now(),
    "updated_at"       timestamp not null             default now(),

    unique (level_ability_id, user_id),

    constraint fk_level_ability
        foreign key (level_ability_id)
            references level_ability (id)
            on update cascade
            on delete cascade,
    constraint fk_user
        foreign key (user_id)
            references "user" (id)
            on update cascade
            on delete cascade
);

select diesel_manage_updated_at('ability');
select diesel_manage_updated_at('level_ability');
select diesel_manage_updated_at('user_level_ability');
