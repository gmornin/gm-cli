use std::{collections::HashMap, io::stdin, process};

pub fn yes(map: &HashMap<String, String>) {
    println!("Are you sure you want to do that?");
    println!("(Press enter to continue, Ctrl + C to exit)");

    if map.get("yes").is_some() {
        println!();
        return;
    }

    stdin().read_line(&mut String::new()).unwrap();
}

pub fn yes_msg(s: &str, map: &HashMap<String, String>) {
    println!("{s}");
    println!("(Press enter to continue, Ctrl + C to exit)");

    if map.get("yes").is_some() {
        println!();
        return;
    }
    stdin().read_line(&mut String::new()).unwrap();
}

pub fn type_yes(map: &HashMap<String, String>) {
    println!("Are you sure you want to do that?");
    println!("(Type \"yes\" to continue, Ctrl + C to exit)");

    if map.get("yes").is_some() {
        println!();
        return;
    }

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    if input.trim() != "yes" {
        process::exit(-1);
    }
}
