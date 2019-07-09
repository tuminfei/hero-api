# hero-api

This project is a simple webserver built with [Rust](https://www.rust-lang.org)
and [Rocket](https://rocket.rs).
It comes with a simple unit test and TravisCI integration.

If you've already set up your MySQL database and have run migrations with Diesel, you can just run `make` to start up the server.
Otherwise, continue reading.

## Getting Started
You can get this project up and running in four easy steps.

### Step 1: Setting up the environment
To download the latest nightly build of Rust and its package manager, [cargo](https://doc.rust-lang.org/cargo/), run
```bash
make setup
```
(Rocket requires us to use nightly builds of Rust). 

### Step 2: Creating the MySQL database
The next step is to install/download MySQL and create a database.

Typically, fresh installs come with a no-password root user. To log in as root on the command line, run
```
mysql -uroot
```

Then, in the MySQL REPL, run:
```
mysql> CREATE DATABASE hero;
Query OK, 1 row affected (0.07 sec)

mysql> CREATE USER IF NOT EXISTS 'hero'@'localhost' IDENTIFIED BY 'hero';
Query OK, 0 rows affected (0.07 sec)

mysql> GRANT ALL ON hero.* TO 'hero'@'localhost';
Query OK, 0 rows affected (0.11 sec)

mysql> exit
Bye
```

From the command line, you can log in as the `hero` user you just created with:
```
mysql -uhero -phero -Dhero
```

### Step 3: Running the database migrations
Next, we need to create a table in our database.
We do this with [Diesel](http://diesel.rs).
Diesel serves two purposes: to run our database migrations and serve as our ORM.
You can install the Diesl CLI with
```
cargo install diesel_cli --no-default-features --features "postgres mysql"
```

Then you can create the database tables with 
```bash
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel migration run
```

### Step 4: Running the server
To start the server, simply run
```bash
# Start the server!
make

# Hit an endpoint!
curl http://localhost:8000/hero
```

## Contributing
### Diesel migrations
When I was first setting up Diesel, I had to have it generate a `./migrations` directory with:
```bash
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel setup
```

Since we already have this directory, you won't have to run this again.

Whenever you want to make schema changes, you can tell Diesel to generate both an "up" script and a "down" script.
The former will get run when you migrate forward; the latter will be used to revert schema and database changes. 

```
# create migration scripts (both up and down)
diesel migration generate create_hero_table

# list migrations
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel migration list

# run migrations
env DATABASE_URL=mysql://hero:hero@localhost/hero diesel migration run
```

### Running tests
```bash
make test
```

### Formatting code
First, clone [rustfmt](https://github.com/rust-lang/rustfmt) and run `cargo install --path . --force`.
That should install `rustfmt` in your `~/.cargo/bin`.
You should be able to format files after that:
```bash
make fmt
```
