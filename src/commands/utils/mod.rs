use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod clean;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([("clean", Box::new(clean::clean) as CommandFnType)])
}
