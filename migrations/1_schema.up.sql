create table if not exists company (
    id serial primary key,
    name text,
    email text,
    password text
);

create table if not exists customer (
    id serial primary key,
    company_id int,
    name text,
    phone text,
    address text
);

create table if not exists customer_transactions (
    id serial primary key,
    customer_id int ,
    amount int,
    date_added timestamp
);