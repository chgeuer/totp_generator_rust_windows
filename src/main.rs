use std::env;
use std::io::{stdin, Read};
use std::process::exit;

// #[macro_use]
extern crate json;

extern crate base32;
use base32::{decode, Alphabet::RFC4648};

extern crate oath;
use oath::{totp_raw_now, HashType::SHA1};

extern crate clipboard_win;
use clipboard_win::set_clipboard_string;

fn get_stdin() -> String {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => input,
        Err(_) => { println!("Could not read secret"); exit(1); },
    }
}

// fn get_secret_from_stdin() -> String {
//     get_stdin().trim().to_string()
// }

fn json_from_stdin() -> json::JsonValue {
    let d = get_stdin();
    json::parse(&d[..]).unwrap()
}

fn navigate_json<'a>(data : &'a json::JsonValue, path : &[String]) -> Option<&'a str> {
    match path.len() {
        0 => data.as_str(),
        _ => navigate_json(&data[&path[0]], &path[1..])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let keys_json = json_from_stdin();
    let key_str = match navigate_json(&keys_json, &args[1..]) {
        Some(v) => v,
        None => { println!("Could not find key in JSON"); exit(1); },
    };

    let key = match decode(RFC4648 { padding: false }, key_str) {
        Some(v) => v,
        None => { println!("Could not decode base32 string \"{}\"", key_str); exit(1); },
    };

    // let algo = "SHA1".to_string();
    // let hash = match &algo.to_lowercase()[..] {
    //     "sha1" => SHA1,
    //     "sha256" => oath::HashType::SHA256,
    //     "sha512" => oath::HashType::SHA512,
    //     _ => { println!("Unknown hash \"{}\"", &algo); exit(1); },
    // };
    // let code = format!("{:<06}", totp_raw_now(&key, 6, 0, 30, &hash));

    let code = format!("{:<06}", totp_raw_now(&key, 6, 0, 30, &SHA1));

    set_clipboard_string(&code).expect("Success");
    println!("Copied code for {} to clipboard", &args[1..].join("/"));
}