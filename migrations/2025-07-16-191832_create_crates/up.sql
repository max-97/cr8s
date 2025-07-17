create table crates (
    id serial primary key,
    rustacean_id integer not null references rustaceans(id),
    code varchar(64) not null,
    name varchar(128) not null,
    versio varchar(64) not null,
    description text,
    created_at timestamp default now() not null
);
