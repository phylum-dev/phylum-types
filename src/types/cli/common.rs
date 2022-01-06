use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ProjectId = Uuid;
pub type JobId = Uuid;
pub type UserId = Uuid;
pub type Key = Uuid;
pub type PackageId = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Complete,
    Incomplete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    None,
    Warn,
    Break,
}
