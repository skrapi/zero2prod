# Following along with Zero2Prod

[![Rust](https://github.com/skrapi/zero2prod/actions/workflows/rust.yml/badge.svg)](https://github.com/skrapi/zero2prod/actions/workflows/rust.yml)


## Testing

Run and follow the instructions to install psql and sqlx-cli.
```sh
./scripts/init_db.sh
```

## Generate sqlx-data.json
```sh
cargo sqlx prepare -- --lib
```
