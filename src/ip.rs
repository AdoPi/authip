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
    pub redis_con: Arc<Mutex<redis::Connection>>,
}

impl AppState {
    pub fn init() -> AppState {
        let client = redis::Client::open("redis://127.0.0.1:6379");
        let c = client.unwrap_or_else(|err| {
            panic!("Error trying to connect to Redis {:?}",err);
        });
        let con = c.get_connection().unwrap_or_else(|err| {
            panic!("Error trying to connect to Redis {:?}",err);
        });
        AppState {
            ip_db: Arc::new(Mutex::new(Vec::new())),
            redis_con: Arc::new(Mutex::new(con)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
