```
    ██   ██ ███████ ███████ ██   ██  █████  ██████
    ██   ██ ██      ██      ██   ██ ██   ██ ██   ██
    ███████ █████   ███████ ███████ ███████ ██████
    ██   ██ ██           ██ ██   ██ ██   ██ ██   ██
    ██   ██ ███████ ███████ ██   ██ ██   ██ ██████
```

![alt Daemon](https://img.shields.io/badge/Type-Web_service-red.svg)
![alt Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![alt Binary](https://img.shields.io/badge/Architecture-binary-green.svg)
![alt Failed](https://img.shields.io/badge/Failed-👎_0-red.svg)
![alt Passed](https://img.shields.io/badge/Passed-👍_0-green.svg)
![alt Version](https://img.shields.io/badge/version-0.1.0_ALPHA-blue.svg)

# Heshab

A simple multi-tenant SaaS that helps companies keep track of their customer credit or "tabs".
Every customer "transaction" can either be a "paid" transaction OR a "credit" transaction.
The SaaS keeps a running total for each customer.

## Features:

- Company Login (all customer data is per company account)
- Simple main search box for customer
- no search results provides option to create new customer
- once a customer is clicked, it shows table of all transactions
- a new transaction of paid/credit can be added or deleted
- each transaction is timestamped

## Building

Make sure you have `sqlx-cli` installed so that you can manually run migrations to make sure that the database schema is in place. Otherwise compilations will fail :|

```
$ cargo install sqlx-cli
$ sqlx migrate run
$ cargo build
```

## Environment Variables

| Variable     | Description          | Example                              |
| ------------ | -------------------- | ------------------------------------ |
| DATABASE_URL | Pg connection string | postgres://dev:pass@localhost/heshab |
| JWT_KEY      | JWT signing key      | 1f726148-499f-4e5b-b164-1e76ff223af1 |
