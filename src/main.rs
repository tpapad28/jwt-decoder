use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Local, TimeZone};
use colored::Colorize;
use json;
use json::JsonValue;
use std::env;
use std::ops::Index;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let parts: Vec<&str> = token.split(".").collect();
    assert_eq!(parts.len(), 3);

    let header = parts[0];
    let payload = parts[1];

    let header_json = decode(header);
    let header_parsed_doc = pretty_parse(header_json.as_str());
    println!("\nHeader:\n{}", header_parsed_doc.pretty);

    let payload_json = decode(payload);
    let payload_parsed_doc = pretty_parse(payload_json.as_str());
    println!("\nPayload:\n{}", payload_parsed_doc.pretty);

    println!("\n{}\n", "Notes:".red().underline());

    let now = Local::now();
    let issued_at_result = display_date(&payload_parsed_doc.data, "iat", "Issued at:");

    match issued_at_result {
        Ok(issued_at) => {
            let exp_diff = now.signed_duration_since(issued_at);
            if exp_diff.num_days() > 0 {
                println!("Issued {} days ago", exp_diff.num_days());
            } else {
                println!("Issued in {} days", exp_diff.num_days().abs());
            }
        }
        Err(_e) => (),
    }

    let end_date_result = display_date(&payload_parsed_doc.data, "exp", "Expires at:");
    match end_date_result {
        Ok(end_date) => {
            let exp_diff = end_date.signed_duration_since(now);
            if exp_diff.num_days() > 0 {
                println!("Expires in {} days", exp_diff.num_days());
            } else {
                println!("Expired {} days ago", exp_diff.num_days().abs());
            }
        }
        Err(_e) => (),
    };
}

fn display_date(
    payload: &JsonValue,
    property: &str,
    msg_format: &str,
) -> Result<DateTime<Local>, bool> {
    match payload.index(property).as_i64() {
        Some(v) => {
            let timestamp = v;
            let effective_date = Local.timestamp_opt(timestamp, 0).unwrap();
            println!("{} {}", msg_format, effective_date);
            Ok(effective_date)
        }
        None => {
            println!("Failed to use {}", property);
            Err(false)
        }
    }
}

fn decode(data: &str) -> String {
    let data_bytes = general_purpose::URL_SAFE
        .decode(&data)
        .unwrap_or_else(|_e| general_purpose::URL_SAFE_NO_PAD.decode(&data).unwrap());
    return String::from_utf8(data_bytes).unwrap();
}

fn pretty_parse(json: &str) -> ParsedDoc {
    let parsed = json::parse(&json).unwrap();
    let pretty = json::stringify_pretty(parsed.clone(), 2);
    return ParsedDoc {
        data: parsed,
        pretty,
    };
}

struct ParsedDoc {
    data: JsonValue,
    pretty: String,
}
