//! This module contains types for working with project data.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::ProjectId;
use super::job::{JobDescriptor, JobResponse};
use super::package::PackageType;

// Response type for the API /projects/<project id> endpoint.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thresholds: ProjectThresholds,
    pub stats: ProjectStats,
    pub latest_job: Option<JobResponse>,
}

/// Summary response for a project
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct ProjectSummaryResponse {
    /// The project id
    pub id: ProjectId,
    /// The project name
    pub name: String,
    /// When the project was updated
    pub updated_at: DateTime<Utc>,
    /// When the project was created
    pub created_at: DateTime<Utc>,
    /// The ecosystem of the project; determined by its latest job
    pub ecosystem: Option<PackageType>,
    /// The project's group's name, if this is a group project
    pub group_name: Option<String>,
}

/// A more detailed project response
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectDetailsResponse {
    /// The project id
    pub id: ProjectId,
    /// The project name
    pub name: String,
    /// The project ecosystem / package type
    pub ecosystem: Option<PackageType>,
    /// The configured risk cutoff thresholds for the project
    pub thresholds: ProjectThresholds,
    /// Most recent analysis job runs
    pub jobs: Vec<JobDescriptor>,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LicensesStats {
    pub counts: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueStatusCounts {
    pub untagged: u32,
    pub will_fix: u32,
    pub accept: u32,
    pub not_relevant: u32,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueStatusStats {
    pub counts: IssueStatusCounts,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DependenciesCounts {
    pub total: u32,
    pub num_incomplete: u32,
    pub above_threshold: u32,
    pub below_threshold: u32,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DependenciesStats {
    pub counts: DependenciesCounts,
}

#[derive(Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStats {
    pub licenses: LicensesStats,
    pub issue_status: IssueStatusStats,
    pub dependencies: DependenciesStats,
}

/// Risk cut off thresholds for a project.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectThresholds {
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
    pub malicious: f32,
    pub total: f32,
    pub vulnerability: f32,
}

/// Request to create a project.
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
