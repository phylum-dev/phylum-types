//! This module contains types for working with project data
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::ProjectId;
use super::package::PackageType;

/// Summary response for a project
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct ProjectSummaryResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: ProjectId,
    /// When the project was updated
    pub updated_at: DateTime<Utc>,
    /// When the project was created
    pub created_at: DateTime<Utc>,
    /// The ecosystems of the project; determined by its latest job
    #[serde(default)]
    pub ecosystems: Vec<PackageType>,
    /// The project's group's name, if this is a group project
    pub group_name: Option<String>,
    /// The project's repository location
    pub repository_url: Option<String>,
}

/// Request to create a project
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct CreateProjectRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
}

pub type UpdateProjectRequest = CreateProjectRequest;

/// Response of a create project request
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct CreateProjectResponse {
    /// The id of the newly created project
    pub id: ProjectId,
}

pub type UpdateProjectResponse = CreateProjectResponse;
