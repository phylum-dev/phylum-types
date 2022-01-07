use serde::{Deserialize, Serialize};

use super::common::ProjectId;
use super::job::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projecct {
    pub score: u32,
    pub passing: bool,
    pub name: String,
    pub id: ProjectId,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectThresholds {
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
    pub malicious: f32,
    pub total: f32,
    pub vulnerability: f32,
}

/// GET /projects/overview
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectGetRequest {
    pub name: String,
    pub id: String,
    pub updated_at: String,
    /* TODO: Need to update request manager to include thresholds with this
     *       response.
     *pub thresholds: ProjectThresholds, */
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectGetResponse {
    pub id: ProjectId,
}

/// GET /projects/<project-id>
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectGetDetailsRequest {
    pub name: String,
    pub id: String,
    pub ecosystem: String,
    pub thresholds: ProjectThresholds,
    pub jobs: Vec<JobDescriptor>,
}

/// PUT /projects
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCreateResponse {
    pub id: ProjectId,
}