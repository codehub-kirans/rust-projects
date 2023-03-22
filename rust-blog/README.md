# A simple Rust based blog server using Axum, Tokio & PostgreSQL
A simple rust based blog that serves markdown posts from a postgres server.
Can be enhanced further to serve pages dynamically.

## Prerequisites

### PostgresSQL
```
Download and install PostgresSQL from (https://www.postgresql.org/download/)
```

### Setup a user and database (inside of psql run the following commands with your own username and password)
```
CREATE ROLE myuser LOGIN PASSWORD 'mypass';
CREATE DATABASE mydb WITH OWNER = myuser;
\q
```

### Create a table that will store posts
```
CREATE TABLE myposts(
post_id SERIAL PRIMARY KEY,
post_date DATE NOT NULL DEFAULT CURRENT_DATE,
post_title TEXT,
post_body TEXT);
```

### Insert a markdown post into the database to list in the homepage
```
cargo run --bin markd "First Post" src/bin/post.md
```

### Run the Rust web server
```
cargo run --bin rust-blog
```

