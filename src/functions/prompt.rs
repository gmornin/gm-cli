use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use rpassword::read_password;

pub fn prompt(s: &str) -> String {
    if s.is_empty() {
        print!("Enter a value: ");
    } else {
        print!("{s}: ");
    }

    stdout().flush().unwrap();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

pub fn prompt_not_present(msg: &str, key: &str, map: &mut HashMap<String, String>) {
    match map.get(key) {
        Some(val) => println!("{msg}: {val}"),
        None => {
            map.insert(key.to_string(), prompt(msg));
        }
    }
}

pub fn prompt_cmd() -> Vec<String> {
    print!("> ");
    stdout().flush().unwrap();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').map(str::to_string).collect()
}

pub fn prompt_password(s: &str) -> String {
    if s.is_empty() {
        print!("Password: ");
    } else {
        print!("{s}: ");
    }

    stdout().flush().unwrap();

    read_password().unwrap()
}

pub fn prompt_password_not_present(msg: &str, key: &str, map: &mut HashMap<String, String>) {
    match map.get(key) {
        Some(_val) => println!("{msg}:"),
        None => {
            map.insert(key.to_string(), prompt_password(msg));
        }
    }
}
