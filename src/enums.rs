use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum MoveBehaviour {
    PointedToPlayer,
}
