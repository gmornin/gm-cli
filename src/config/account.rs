use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::traits::ConfigTriat;

#[derive(Serialize, Deserialize, Default)]
pub struct AccountConfig {
    pub id: String,
    pub instance: String,
    pub token: String,
}

impl ConfigTriat for AccountConfig {
    const NAME: &'static str = "account";
}

impl AccountConfig {
    pub fn is_loggedin(&self) -> bool {
        !(self.id.is_empty() || self.instance.is_empty() || self.token.is_empty())
    }

    pub fn is_loggedin_map(map: &HashMap<String, String>) -> bool {
        map.contains_key("id")
            && !map.get("id").unwrap().is_empty()
            && map.contains_key("instance")
            && !map.get("instance").unwrap().is_empty()
            && map.contains_key("token")
            && !map.get("token").unwrap().is_empty()
    }
}
