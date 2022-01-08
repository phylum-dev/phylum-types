//! This module contains types for working with project data

use serde::{Deserialize, Serialize};

use super::common::ProjectId;
use super::job::*;

/// Rick cut off thresholds for a project
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectThresholds {
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
    pub malicious: f32,
    pub total: f32,
    pub vulnerability: f32,
}

/// Summary response for a project
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSummaryResponse {
    /// The project name
    pub name: String,
    /// The project id
    pub id: String,
    /// When the project was updated
    // TODO Fix, make consistent with other timestamps in the api
    pub updated_at: String,
    /* TODO: Need to update request manager to include thresholds with this
     *       response.
     *pub thresholds: ProjectThresholds, */
}

/// A more detailed project response
#[derive(Debug, Serialize, Deserialize)]
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

/// Response of a create project request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {
    /// The id of the newly created project
    pub id: ProjectId,
}
