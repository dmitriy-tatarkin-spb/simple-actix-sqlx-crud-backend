An example CRUD backend written in Rust with actix_web, sqlx and JWT authentication.

Prepare environment:

It uses postgresql database, so you should set enviroment variable for it. For example,

`DATABASE_URL=postgres://postgres:postgres@localhost/todos`

Also, you need to set *SECRET_KEY* env for your JWT-token to work with backend securely.
You can generate token with help of [jwt.io](https://jwt.io).

Futher set *HOST*, *PORT* envs for your backend hostname and port.

Run migration to create required table:
```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

Run backend:
```bash
cargo sqlx prepare
cargo run
```
