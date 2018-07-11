#[macro_use]
extern crate diesel_migrations;
extern crate clap;
extern crate diesel;

embed_migrations!("./migrations");

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use diesel::pg::PgConnection;
use diesel::Connection;

pub fn main() {
    let matches = App::new("Big Neon DB CLI")
        .author("Big Neon")
        .about("Command Line Interface for creating and migrating the Big Neon database")
        .subcommand(
            SubCommand::with_name("migrate")
                .about("Migrates the database to the latest version")
                .arg(
                    Arg::with_name("connection")
                        .short("c")
                        .takes_value(true)
                        .help("Connection string to the database"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("migrate", Some(matches)) => create_and_migrate_db(matches),
        _ => unreachable!("The cli parser will prevent reaching here"),
    }
}

fn create_and_migrate_db(matches: &ArgMatches) {
    let conn_string = matches.value_of("connection").unwrap();
    let parts: Vec<&str> = conn_string.split("/").collect();
    let db = parts.last().unwrap();
    let db = str::replace(db, "'", "''");
    let postgres_conn_string = str::replace(conn_string, &db, "postgres");
    let connection = PgConnection::establish(&postgres_conn_string).unwrap();

    match connection.execute(&format!("CREATE DATABASE \"{}\"", db)) {
        Ok(_o) => println!("Creating database"),
        Err(_e) => println!("Database already exists"),
    }
    println!("Migrating database");

    let connection = PgConnection::establish(conn_string).unwrap();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Migration failed");
}
