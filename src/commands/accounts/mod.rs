use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod create;
mod delete;
mod login;
mod logout;
mod regen;
mod rename;
mod status;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("create", Box::new(create::create) as CommandFnType),
        ("login", Box::new(login::login) as CommandFnType),
        ("logout", Box::new(logout::logout) as CommandFnType),
        ("regen", Box::new(regen::regen) as CommandFnType),
        ("rename", Box::new(rename::rename) as CommandFnType),
        ("delete", Box::new(delete::delete) as CommandFnType),
        ("status", Box::new(status::status) as CommandFnType),
    ])
}
