use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

use crate::types::user_settings::Threshold;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CorePreferences {
    /// The default label to use when none is supplied.
    pub default_label: Option<String>,
    /// The risk thresholds to apply.
    pub thresholds: RiskThresholds,
    /// Project specific ignored issues.
    pub ignored_issues: Option<Vec<IgnoredIssue>>,
}

/// The preferences for a given project.
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPreferences {
    /// The id of the project these preferences apply to.
    pub project_id: Uuid,
    /// The preference settings
    pub preferences: CorePreferences,
}

/// Capture the project threshold settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct RiskThresholds {
    pub total: Threshold,
    pub author: Threshold,
    pub engineering: Threshold,
    pub license: Threshold,
    #[serde(alias = "malicious")]
    #[serde(rename = "maliciousCode")]
    pub malicious: Threshold,
    pub vulnerability: Threshold,
}

/// Issues ignored from package score
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct IgnoredIssue {
    pub id: String,
    pub tag: String,
    pub reason: String,
}
