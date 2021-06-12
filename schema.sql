-- noinspection SqlDialectInspectionForFile

-- noinspection SqlNoDataSourceInspectionForFile

CREATE DATABASE test;

\connect test;

CREATE TABLE IF NOT EXISTS orders (
    id                SERIAL PRIMARY KEY,
    item_id           INTEGER NOT NULL,
    table_number      INTEGER NOT NULL,
    preparation_time  INTEGER NOT NULL DEFAULT 10,
    status            VARCHAR NOT NULL DEFAULT 'pending'
);

CREATE DATABASE restaurant;

\connect restaurant;

CREATE TABLE IF NOT EXISTS orders (
    id                SERIAL PRIMARY KEY,
    item_id           INTEGER NOT NULL,
    table_number      INTEGER NOT NULL,
    preparation_time  INTEGER NOT NULL DEFAULT 10,
    status            VARCHAR NOT NULL DEFAULT 'pending'
);

CREATE TABLE IF NOT EXISTS items (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    price       INTEGER NOT NULL DEFAULT 100
);