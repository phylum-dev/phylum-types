//! This module contains types involved with handling phylum processing jobs.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::*;
use super::project::*;
use crate::types::package::{PackageDescriptor, PackageStatus, PackageStatusExtended, PackageType};

/// When a job is completed, and some requirement is not met ( such as quality
/// level ), what action should be taken?
/// In the case of the CLI, the value of this result is used to determine if the
/// CLI should print a warning, or exit with a non-zero exit code.
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    None,
    Warn,
    Break,
}

/// Metadata about a job
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
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
    // don't deprecate until `ecosystems` is live.
    //#[deprecated = "Use `ecosystems` to support multiple ecosystems."]
    pub ecosystem: String,
    pub ecosystems: Vec<String>,
    #[serde(default)]
    pub num_incomplete: u32,
}

/// Submit Package for analysis
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct SubmitPackageRequest {
    /// The 'type' of package, NPM, RubyGem, etc
    // don't deprecate until `ecosystems` is live.
    //#[deprecated = "No longer used."]
    #[serde(rename = "type")]
    pub package_type: PackageType,
    /// The subpackage dependencies of this package
    pub packages: Vec<PackageDescriptor>,
    /// Was this submitted by a user interactively and not a CI?
    pub is_user: bool,
    /// The id of the project this top level package should be associated with
    pub project: ProjectId,
    /// A label for this package. Often it's the branch.
    pub label: String,
    /// The group that owns the project, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// Initial response after package has been submitted
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct SubmitPackageResponse {
    /// The id of the job processing the package
    pub job_id: JobId,
}

/// Represents a response that summarizes the output of all current jobs
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AllJobsStatusResponse {
    /// A description of the latest jobs
    pub jobs: Vec<JobDescriptor>,
    /// Total jobs run
    pub total_jobs: u32,
    pub count: u32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum JobStatusResponseVariant {
    // Serde returns the one that deserializes successfully first, so most complicated goes first
    Extended(JobStatusResponse<PackageStatusExtended>),
    Basic(JobStatusResponse<PackageStatus>),
}

/// Data returned when querying the job status endpoint
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct JobStatusResponse<T> {
    /// The id of the job processing the top level package
    pub job_id: JobId,
    /// The language ecosystem
    // don't deprecate until `ecosystems` is live.
    //#[deprecated = "Use `ecosystems` to support multiple ecosystems."]
    pub ecosystem: String,
    /// The language ecosystem
    pub ecosystems: Vec<String>,
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
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct CancelJobResponse {
    pub msg: String,
}
