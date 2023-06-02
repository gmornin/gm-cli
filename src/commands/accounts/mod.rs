use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod create;
mod login;
mod logout;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("create", Box::new(create::create) as CommandFnType),
        ("login", Box::new(login::login) as CommandFnType),
        ("logout", Box::new(logout::logout) as CommandFnType),
    ])
}
