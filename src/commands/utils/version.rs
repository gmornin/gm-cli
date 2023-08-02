use std::{collections::HashMap, error::Error};

pub fn version(
    _map: HashMap<String, String>,
    _args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok("You're welcome!".to_string())
}
