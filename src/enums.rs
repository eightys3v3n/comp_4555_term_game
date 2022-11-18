use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum MoveBehaviour {
    PointedToPlayer,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Playing,
    Paused,
}
