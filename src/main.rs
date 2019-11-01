mod cri_api;
mod models;

use crate::cri_api::{get_all_users, get_jwt};
use clap::{value_t_or_exit, App, Arg};
use dotenv::dotenv;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let matches = App::new("KekeCRI")
        .version("0.1.0")
        .author("Hugo D. <hugo.delahousse@gmail.com>")
        .about("Downloads a mapping of emails to login for EPITA's CRI")
        .arg(
            Arg::with_name("username")
                .short("u")
                .value_name("CRI_USERNAME")
                .env("KEKECRI_USERNAME")
                .required(true)
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .value_name("CRI_PASSWORD")
                .env("KEKECRI_PASSWORD")
                .required(true)
        )
        .arg(
            Arg::with_name("batch-size")
                .short("s")
                .env("KEKECRI_BATCH_SIZE")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT_FILE")
                .short("o")
        )
        .get_matches();

    let username = matches.value_of("username").unwrap().to_string();
    let password = matches.value_of("password").unwrap().to_string();
    let batch_size = value_t_or_exit!(matches.value_of("batch-size"), u32);

    let mut output = BufWriter::new(match matches.value_of("output") {
        Some(value) => Box::new(File::create(value)?) as Box<dyn Write>,
        None => Box::new(std::io::stdout()) as Box<dyn Write>,
    });

    let authenticator = move || Ok(format!("JWT {}", get_jwt(&username, &password)?));

    let users = get_all_users(batch_size, &authenticator);

    // Use BTreeMap instead of hashMap to have an ordered result
    let mapping: BTreeMap<String, String> = users?
        .iter()
        .map(|user| (user.mail.clone(), user.login.clone()))
        .collect();

    output.write_all(serde_json::to_string_pretty(&mapping)?.as_bytes())?;
    output.write(b"\n")?;

    Ok(())
}
