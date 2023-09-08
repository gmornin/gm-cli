use std::{collections::HashMap, error::Error};

pub fn map_args(
    map: &mut HashMap<String, String>,
    keys: &[&str],
    args: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    if keys.len() < args.len() {
        return Err(crate::error::Error::StrErr("Too many arguments").into());
    }

    keys.iter().zip(args).for_each(|(key, arg)| {
        map.insert(key.to_string(), arg);
    });

    Ok(())
}
