use std::collections::HashMap;
use tokio::sync::Mutex;
use webauthn_rs::prelude::*;

pub struct AuthenticationState {
    // Map of unique user IDs to their registration details
    state_map: Mutex<HashMap<String, (Uuid, PasskeyAuthentication)>>,
}

impl AuthenticationState {
    pub fn new() -> Self {
        AuthenticationState {
            state_map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn insert(&self, data: (Uuid, PasskeyAuthentication)) {
        let mut map = self.state_map.lock().await;
        map.insert("auth_state".to_string(), data);
    }

    pub async fn get(&self, user_unique_id: String) -> Option<(Uuid, PasskeyAuthentication)> {
        let map = self.state_map.lock().await;
        map.get(&user_unique_id).cloned()
    }

    pub async fn remove(&self, user_unique_id: String) {
        let mut map = self.state_map.lock().await;
        map.remove(&user_unique_id);
    }
}
