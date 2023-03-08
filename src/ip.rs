use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ip {
    pub ipv4: String,
    pub desc: String,
}


// This should use redis
pub struct AppState {
    pub ip_db: Arc<Mutex<Vec<Ip>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            ip_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
