create table overlays
(
    id serial not null,
    created_by varchar(16) not null
        constraint overlays_users_id_fk
            references users
            on delete cascade,
    for_user varchar(25) not null,
    secret varchar(30) not null
);

create unique index overlays_id_uindex
    on overlays (id);

alter table overlays
    add constraint overlays_pk
        primary key (id);
