#![allow(clippy::module_inception)]

use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod jobs;
mod unqueue;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("jobs", Box::new(jobs::jobs) as CommandFnType),
        ("unqueue", Box::new(unqueue::unqueue) as CommandFnType),
    ])
}
