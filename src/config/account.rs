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
