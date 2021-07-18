create table users
(
    id varchar(16) not null
        constraint users_pk
            primary key,
    name varchar(25) not null,
    access_token varchar(30) not null,
    refresh_token text not null,
    scopes text not null
);

alter table users owner to imglaze;

create unique index users_id_uindex
    on users (id);
