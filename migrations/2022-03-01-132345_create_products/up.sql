create table products
(
    id   integer not null
        constraint products_pk
            primary key autoincrement,
    name text    not null
);

create unique index products_Id_uindex
    on products (Id)