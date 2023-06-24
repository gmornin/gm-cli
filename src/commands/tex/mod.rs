use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod profile;
mod set_profile;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([("profile", Box::new(profile::profile) as CommandFnType), ("pfedit", Box::new(set_profile::set_profile) as CommandFnType)])
}
