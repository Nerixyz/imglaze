create table config
(
    key varchar(16) not null,
    value jsonb not null
);

create unique index config_key_uindex
	on config (key);

alter table config
    add constraint config_pk
        primary key (key);
