use std::{collections::HashMap, env, error::Error, fs};

use log::*;

pub fn clean(_map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let path = dirs::cache_dir().unwrap().join(env!("CARGO_PKG_NAME"));
    info!("Cleaning all items in {:?}", path);
    fs::remove_dir_all(path)?;
    info!("Directory cleared");
    Ok(String::from("Deleted"))
}
