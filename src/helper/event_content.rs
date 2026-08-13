use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct EventContent {
    pub version: u32,
    pub recipients: HashMap<String, String>,
}
