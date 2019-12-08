use atty::{is, Stream};
use clap::ArgMatches;
use rusqlite::{self, params, Connection, NO_PARAMS};
use std::io::{self, Read};
use std::{process, str::FromStr};

use crate::utils;

pub fn add(args: &ArgMatches, conn: Connection) {
    if args.is_present("add_entity") {
        let name = args.value_of("add_entity").unwrap();
        match add_new_entity(conn, name) {
            Ok(()) => println!("entity name `{}` added", name),
            Err(e) => {
                eprintln!("Could not add entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_alias") {
        let alias_args: Vec<_> = args.values_of("add_alias").unwrap().collect();
        let entity_id = u32::from_str(alias_args[0]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an u32");
            process::exit(1);
        });

        match add_alias_to_entity(conn, entity_id, alias_args[1]) {
            Ok(()) => println!(
                "alias `{}` to entity_id `{}` added",
                alias_args[1], entity_id
            ),
            Err(e) => {
                eprintln!("Could not add alias to entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_relation") {
        let alias_args: Vec<_> = args.values_of("add_relation").unwrap().collect();

        let entity_id_a = u32::from_str(alias_args[0]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an u32");
            process::exit(1);
        });

        let entity_id_b = u32::from_str(alias_args[1]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an u32");
            process::exit(1);
        });

        match add_relation_two_entities(conn, entity_id_a, entity_id_b) {
            Ok(()) => println!(
                "relation between entity_id `{}` and entity_id `{}` added",
                entity_id_a, entity_id_b
            ),
            Err(e) => {
                eprintln!("Could not add relation between two entities, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_snippet") {
        let entity_id =
            u32::from_str(args.value_of("add_snippet").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });
        match add_new_snippet(conn, entity_id) {
            Ok(()) => {
                println!("{}", "-".repeat(40));
                println!("new data snippet added to entity id `{}`", entity_id);
            }
            Err(e) => {
                eprintln!("Could not add snippet to entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_relation_snippet") {
        let relation_id = u32::from_str(args.value_of("add_relation_snippet").unwrap())
            .unwrap_or_else(|_err| {
                eprintln!("relation_id must be an u32");
                process::exit(1);
            });
        match add_relation_snippet(conn, relation_id) {
            Ok(()) => {
                println!("{}", "-".repeat(40));
                println!("new data snippet added to relation id `{}`", relation_id);
            }
            Err(e) => {
                eprintln!("Could not add snippet to relation_snippet, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn add_alias_to_entity(conn: Connection, entity_id: u32, name: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO alias (entity_id, name) VALUES
                 (?1, ?2)",
        params![entity_id, name],
    )?;

    Ok(())
}

fn add_new_entity(conn: Connection, name: &str) -> rusqlite::Result<()> {
    conn.execute("INSERT INTO entity default values", NO_PARAMS)?;
    conn.execute(
        "INSERT INTO alias (name, entity_id) VALUES
                 (?1, (SELECT seq from sqlite_sequence where name='entity'))",
        params![name],
    )?;

    Ok(())
}

fn add_relation_two_entities(conn: Connection, id_a: u32, id_b: u32) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO relation (entity_id_a, entity_id_b) VALUES
                 (?1, ?2)",
        params![id_a, id_b],
    )?;

    Ok(())
}

fn add_new_snippet(conn: Connection, entity_id: u32) -> rusqlite::Result<()> {
    // Check if Stdin pipe is open, if it is then these messages will be omitted
    if is(Stream::Stdin) {
        if cfg!(taget_os = "windows") {
            println!("[Type in data for snippet - Termiate by Ctrl-Z and Return (Enter)]");
        } else {
            println!("[Type in data for snippet - Termiate by Return (Enter) and Ctrl-D]");
        }
    }

    let mut data = String::new();
    match io::stdin().read_to_string(&mut data) {
        Ok(_) => {
            data = utils::trim_trailing_newline(&mut data);
        }

        Err(err) => {
            eprintln!("Something went wrong reading input! Error: {}", err);
            process::exit(1);
        }
    }

    conn.execute(
        "INSERT INTO snippet (data, entity_id) VALUES (?1, ?2)",
        params![data, entity_id],
    )?;

    Ok(())
}

fn add_relation_snippet(conn: Connection, relation_id: u32) -> rusqlite::Result<()> {
    // Check if Stdin pipe is open, if it is then these messages will be omitted
    if is(Stream::Stdin) {
        if cfg!(taget_os = "windows") {
            println!("[Type in data for snippet - Termiate by Ctrl-Z and Return (Enter)]");
        } else {
            println!("[Type in data for snippet - Termiate by Return (Enter) and Ctrl-D]");
        }
    }

    let mut data = String::new();
    match io::stdin().read_to_string(&mut data) {
        Ok(_) => {
            data = utils::trim_trailing_newline(&mut data);
        }

        Err(err) => {
            eprintln!("Something went wrong reading input! Error: {}", err);
            process::exit(1);
        }
    }

    conn.execute(
        "INSERT INTO relation_snippet (data, relation_id) VALUES (?1, ?2)",
        params![data, relation_id],
    )?;

    Ok(())
}
