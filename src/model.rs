use std::ops::Index;

use base64::{engine::general_purpose, Engine as _};
use json::JsonValue;

#[derive(Debug, Clone)]
pub struct Header {
    pub typ: String,
    pub alg: String,
    pub json: ParsedJson,
    pub original: String,
}

#[derive(Debug, Clone)]
pub struct Claims {
    //sub: String,
    //iss: String,
    //aud: String,
    pub exp: i64,
    //nbf: u32,
    pub iat: i64,
    pub json: ParsedJson,
    pub original: String,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub header: Header,
    pub claims: Claims,
    pub original: String,
}

#[derive(Debug, Clone)]
pub struct ParsedJson {
    pub data: JsonValue,
    pub pretty: String,
}

impl Token {
    pub fn new(original: String) -> Self {
        let parts: Vec<&str> = original.split(".").collect();
        return Token {
            original: original.clone(),
            header: Header::new(parts[0].to_string()),
            claims: Claims::new(parts[1].to_string()),
        };
    }
}

impl Claims {
    fn new(original: String) -> Self {
        let json = decode(&original);

        let exp = match json.data.index("exp").as_i64() {
            Some(v) => Ok(v),
            None => Err(false),
        };
        let iat = match json.data.index("iat").as_i64() {
            Some(v) => Ok(v),
            None => Err(false),
        };

        if iat.is_ok() && exp.is_ok() {
            return Claims {
                exp: exp.unwrap(),
                iat: iat.unwrap(),
                original: original.clone(),
                json,
            };
        } else {
            return Claims {
                exp: -1,
                iat: -1,
                original,
                json: ParsedJson {
                    data: JsonValue::Null,
                    pretty: "".to_string(),
                },
            };
        }
    }
}

impl Header {
    fn new(original: String) -> Self {
        let json = decode(&original);
        let typ = match json.data.index("typ").as_str() {
            Some(v) => Ok(v),
            None => Err(false),
        };
        let alg = match json.data.index("alg").as_str() {
            Some(v) => Ok(v),
            None => Err(false),
        };

        if typ.is_ok() && alg.is_ok() {
            return Header {
                typ: typ.unwrap().to_string(),
                alg: String::from(""),
                original,
                json,
            };
        } else {
            return Header {
                alg: "".to_string(),
                typ: "".to_string(),
                original: "".to_string(),
                json: ParsedJson {
                    data: JsonValue::Null,
                    pretty: "".to_string(),
                },
            };
        }
    }
}

fn decode(data: &str) -> ParsedJson {
    let data_bytes = general_purpose::URL_SAFE
        .decode(&data)
        .unwrap_or_else(|_e| general_purpose::URL_SAFE_NO_PAD.decode(&data).unwrap());
    let decoded = String::from_utf8(data_bytes).unwrap();
    return pretty_parse(&decoded);
}

fn pretty_parse(json: &str) -> ParsedJson {
    let parsed = json::parse(&json).unwrap();
    let pretty = json::stringify_pretty(parsed.clone(), 2);
    return ParsedJson {
        data: parsed,
        pretty,
    };
}
