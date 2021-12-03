use std::{env, path::Path};

use clap::clap_app;

fn main() {
    let args = clap_app!(cache_input =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "Cache AOC 2021 input to the local filesystem")
        (@arg SESSION: -s --session +takes_value "A logged in session cookie auth token")
        (@arg PACKAGE: +required "The package (day) to download input for")
    )
    .get_matches();

    let session = match args.value_of("SESSION") {
        Some(session) => session.to_owned(),
        None => env::var("AOC_2021_SESSION")
            .expect("Provide a session token via args or AOC_2021_SESSION env"),
    };

    let package = args.value_of("PACKAGE").unwrap();

    match cache_input(&session, package) {
        Ok(msg) => println!("Ok: {}", msg),
        Err(msg) => eprintln!("Err: {}", msg),
    };
}

fn cache_input(session: &str, package: &str) -> Result<String, String> {
    if !Path::new(package).exists() {
        return Err("package dir doesn't exist, exiting".into());
    }

    let fq_path = format!("{}/input", package);
    let path = Path::new(&fq_path);
    if path.exists() {
        return Ok("path already exists, nothing to do".into());
    }

    let (_, day) = package
        .split_once("day")
        .ok_or("package should be in the format 'day??'")?;

    let input = ureq::get(&format!(
        "https://adventofcode.com/2021/day/{}/input",
        day.parse::<u8>().map_err(|_| "day number is not valid")?
    ))
    .set("Cookie", &format!("session={}", session))
    .call()
    .unwrap()
    .into_string()
    .map_err(|err| err.to_string())?;

    std::fs::write(&path, input).map_err(|err| err.to_string())?;

    Ok(format!("cached file to {}", path.to_str().unwrap()))
}
