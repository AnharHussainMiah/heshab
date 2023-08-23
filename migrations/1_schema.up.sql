create table if not exists company (
    id serial primary key,
    name text,
    email text,
    password text,
    reset_token text,
    reset_added timestamp
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
    company_id int,
    customer_id int ,
    amount int,
    date_added timestamp
);