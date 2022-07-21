-- Your SQL goes here

CREATE TABLE todos (
     id serial primary key,
     title boolean not null default false,
     checked boolean not null default false
);