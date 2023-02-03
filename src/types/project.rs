//! This module contains types for working with project data
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::ProjectId;
use super::job::*;
use super::package::PackageType;

/// Rick cut off thresholds for a project
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectThresholds {
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
    pub malicious: f32,
    pub total: f32,
    pub vulnerability: f32,
}

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
    /// The ecosystem of the project; determined by its latest job
    #[deprecated = "Use `ecosystems` to support multiple ecosystems."]
    pub ecosystem: Option<PackageType>,
    /// The ecosystems of the project; determined by its latest job
    #[serde(default)]
    pub ecosystems: Vec<PackageType>,
    /// The project's group's name, if this is a group project
    pub group_name: Option<String>,
}

/// A more detailed project response
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectDetailsResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: String,
    /// The project ecosystem / package type
    #[deprecated = "Use `ecosystems` to support multiple ecosystems."]
    pub ecosystem: String,
    /// The project ecosystems / package types
    #[serde(default)]
    pub ecosystems: Vec<String>,
    /// The configured risk cutoff thresholds for the project
    pub thresholds: ProjectThresholds,
    /// Most recent analysis job runs
    pub jobs: Vec<JobDescriptor>,
}

/// Rquest to create a project
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct CreateProjectRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
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
