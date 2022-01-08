//! This module contains types involved with handling phylum processing jobs.

use serde::{Deserialize, Serialize};

use super::common::*;
use super::package::*;
use super::project::*;

/// When a job is completed, and some requirement is not met ( such as quality
/// level ), what action should be taken?
/// In the case of the CLI, the value of this result is used to determine if the
/// CLI should print a warning, or exit with a non-zero exit code.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    None,
    Warn,
    Break,
}

/// Metadata about a job
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

/// Submit Package for analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitPackageRequest {
    /// The 'type' of package, NPM, RubyGem, etc
    pub r#type: PackageType,
    /// The subpackage dependencies of this package
    pub packages: Vec<PackageDescriptor>,
    /// Was this submitted by a user interactively and not a CI?
    pub is_user: bool,
    /// The id of the project this top level package should be associated with
    pub project: ProjectId,
    /// A label for this package. Often it's the branch.
    pub label: String,
}

/// Initial response after package has been submitted
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSubmissionResponse {
    /// The id of the job processing the package
    pub job_id: JobId,
}

/// Represents a response that summarizes the output of all current jobs
#[derive(Debug, Serialize, Deserialize)]
pub struct AllJobsStatusResponse {
    /// A description of the latest jobs
    pub jobs: Vec<JobDescriptor>,
    /// Total jobs run
    pub total_jobs: u32,
    pub count: u32,
}

/// Data returned when querying the job status endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatusResponse<T> {
    /// The id of the job processing the top level package
    pub job_id: JobId,
    /// The language ecosystem
    /// TODO: How is this different than package type ( npm, etc ) or language?
    pub ecosystem: String,
    /// The id of the user submitting the job
    pub user_id: UserId,
    /// The user email
    pub user_email: String,
    /// The time the job started in epoch seconds
    pub created_at: i64,
    /// The job status
    pub status: Status,
    /// The current score
    pub score: f64,
    pub pass: bool,
    pub msg: String,
    /// The action to take if the job fails
    pub action: Action,
    #[serde(default)]
    /// Dependencies that have not completed processing
    pub num_incomplete: u32,
    /// The last time the job metadata was updated
    pub last_updated: u64,
    /// The id of the project associated with this job
    pub project: String,
    /// The project name
    pub project_name: String,
    /// A label associated with this job, most often a branch name
    pub label: Option<String>,
    /// The currently configured threshholds for this job. If the scores fall
    /// below these thresholds, then the client should undertake the action
    /// spelled out by the action field.
    pub thresholds: ProjectThresholds,
    /// The packages that are a part of this job
    pub packages: Vec<T>,
}

/// Response from canceling a job
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelJobResponse {
    pub msg: String,
}
