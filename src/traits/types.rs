use std::{collections::HashMap, error::Error};

pub type CommandFnType = Box<dyn Fn(HashMap<String, String>) -> Result<String, Box<dyn Error>>>;
