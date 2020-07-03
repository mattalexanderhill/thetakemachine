## The Take Machine!

Welcome to the [take machine](http://www.thetakemachine.com). Built with [rocket.rs](https://rocket.rs/) 🚀!

## Setup and Requirements

Requirements are managed throught cargo, database is postgres and is managed by diesel.
Database can be setup from the command line with diesel cli. To install try

```sh
cargo install diesel_cli --no-default-features --features postgres
```

After creating a local database, you should now be able to setup generate database tables with.
Ensure the `DATABASE_URL` is set in `.env`.

```sh
diesel migration run
```

## Build

Building is managed through cargo, try

```sh
cargo run
```

## Test

Tests are also managed through cargo, try

```sh
cargo test
```
