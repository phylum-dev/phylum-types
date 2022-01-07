use serde::{Deserialize, Serialize};

use super::common::*;
use super::package::*;
use super::project::*;

/// If
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    None,
    Warn,
    Break,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobDescriptor {
    pub job_id: JobId,
    pub project: String,
    pub label: String,
    pub num_dependencies: u32,
    pub score: f64,
    pub packages: Vec<PackageDescriptor>,
    pub pass: bool,
    pub msg: String,
    pub date: String,
    pub ecosystem: String,
    #[serde(default)]
    pub num_incomplete: u32,
}

/// PUT /job
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageRequest {
    pub r#type: PackageType,
    pub packages: Vec<PackageDescriptor>,
    pub is_user: bool,
    pub project: ProjectId,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSubmissionResponse {
    pub job_id: JobId,
}

/// GET /job/auth_status
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatusResponse {
    pub authenticated: bool,
}

/// GET /job/heartbeat
#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub msg: String,
}

/// GET /job
#[derive(Debug, Serialize, Deserialize)]
pub struct AllJobsStatusResponse {
    pub jobs: Vec<JobDescriptor>,
    pub total_jobs: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatusResponse<T> {
    pub job_id: JobId,
    pub ecosystem: String,
    pub user_id: UserId,
    pub user_email: String,
    pub created_at: i64, // epoch seconds
    pub status: Status,
    pub score: f64,
    pub pass: bool,
    pub msg: String,
    pub action: Action,
    #[serde(default)]
    pub num_incomplete: u32,
    pub last_updated: u64,
    pub project: String, // project id
    pub project_name: String,
    pub label: Option<String>,
    pub thresholds: ProjectThresholds,
    pub packages: Vec<T>,
}

/// DELETE /request/packages/<job_id>
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelJobResponse {
    pub msg: String,
}
