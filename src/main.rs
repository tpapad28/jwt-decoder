mod cli;
mod model;

use chrono::{DateTime, Local, TimeZone};
use colored::Colorize;
use std::str;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = cli::Parameters::new(&args).unwrap_or_else(|err| {
        eprintln!("Invalid arguments: {}", err);
        process::exit(1);
    });

    let token = params.jwt;

    let header = token.header;
    let claims = token.claims;

    println!("\nHeader:\n{}", header.json.pretty);

    println!("\nPayload:\n{}", claims.json.pretty);

    println!("\n{}\n", "Notes:".red().underline());

    let now = Local::now();
    let issued_at = display_date(claims.iat, "Issued at:");

    let exp_diff = now.signed_duration_since(issued_at);
    if exp_diff.num_days() > 0 {
        println!("Issued {} days ago", exp_diff.num_days());
    } else {
        println!("Issued in {} days", exp_diff.num_days().abs());
    }

    let end_date = display_date(claims.exp, "Expires at:");
    let exp_diff = end_date.signed_duration_since(now);
    if exp_diff.num_days() > 0 {
        println!("Expires in {} days", exp_diff.num_days());
    } else {
        println!("Expired {} days ago", exp_diff.num_days().abs());
    }
}

fn display_date(ts: i64, msg_format: &str) -> DateTime<Local> {
    let effective_date = Local.timestamp_opt(ts, 0).unwrap();
    println!("{} {}", msg_format, effective_date);
    return effective_date;
}
