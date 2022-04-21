//! This module contains types for working with project data
#[cfg(feature = "dev_api_issue_96")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::ProjectId;
use super::job::*;

/// Rick cut off thresholds for a project
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectThresholds {
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
    pub malicious: f32,
    pub total: f32,
    pub vulnerability: f32,
}

/// Summary response for a project
#[cfg(not(feature = "dev_api_issue_96"))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSummaryResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: String,
    /// When the project was updated
    pub updated_at: String,
    /* TODO: Need to update request manager to include thresholds with this
     *       response.
     *pub thresholds: ProjectThresholds, */
}

/// Summary response for a project
#[cfg(feature = "dev_api_issue_96")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSummaryResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: ProjectId,
    /// When the project was updated
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    /* TODO: Need to update request manager to include thresholds with this
     *       response.
     *pub thresholds: ProjectThresholds, */
}

/// A more detailed project response
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectDetailsResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: String,
    /// The project ecosystem / package type
    pub ecosystem: String,
    /// The configured risk cutoff thresholds for the project
    pub thresholds: ProjectThresholds,
    /// Most recent analysis job runs
    pub jobs: Vec<JobDescriptor>,
}

/// Rquest to create a project
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub group_name: Option<String>
}

pub type UpdateProjectRequest = CreateProjectRequest;

/// Response of a create project request
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {
    /// The id of the newly created project
    pub id: ProjectId,
}

pub type UpdateProjectResponse = CreateProjectResponse;
