use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveBehaviour {
    PointedToPlayer,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AppState {
    MainMenu,
    Playing,
    Paused,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ButtonID {
    NewGame,
    Exit,
}

