use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ProjectId = Uuid;
pub type JobId = Uuid;
pub type UserId = Uuid;
pub type Key = Uuid;
pub type PackageId = String;

/// Did the processing of the Package or Job complete successfully
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Complete,
    Incomplete,
}
