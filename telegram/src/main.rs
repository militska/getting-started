extern crate rustc_serialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const SEND_MESSAGE_ACTION: &str = "sendMessage";

#[derive(RustcDecodable, Debug)]
struct Conf {
    chat_id: Option<String>,
    bot_id: Option<String>,
}

fn main() {
    let message = String::from("hi!");
    let cnf = configure();

    send_message(cnf, message)
}

fn send_message(cnf: Conf, message: String) {
    let message = format_text(message.to_string(), true);

    let mut map = HashMap::new();
    map.insert("chat_id", cnf.chat_id.as_ref().unwrap());
    map.insert("text", &message);

    let client = reqwest::blocking::Client::new();
    let res = client.post(format!( "https://api.telegram.org/bot{}/{}", String::from(cnf.bot_id.unwrap()), String::from(SEND_MESSAGE_ACTION)))
        .json(&map)
        .send()
        .unwrap();

    println!("{}", res.status());
}

fn format_text(mut message: String, modify: bool) -> String {
    if modify {
        message = "message from rust bot \n".to_string() + &message + ";\n\nby militska;";
    }
    return message;
}


fn configure() -> Conf {
    let conf_file = "./src/config.toml";

    let mut zhdl = match File::open(conf_file) {
        Ok(f) => f,
        Err(e) => panic!("Error occurred opening file: {} - Err: {}", conf_file, e)
    };

    let mut zstr = String::new();
    match zhdl.read_to_string(&mut zstr) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let cnf: Conf = toml::decode_str(&zstr).unwrap();

    return cnf
}