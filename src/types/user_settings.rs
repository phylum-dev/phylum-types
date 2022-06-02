//! This module contains types for manipulating user settings data

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Threshold for a given risk
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Threshold {
    // TODO Should this be the Action enum?
    pub action: String,
    /// Is this threshold active
    pub active: bool,
    /// The risk threshold cutoff
    pub threshold: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserProject {
    pub thresholds: HashMap<String, Threshold>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Setting {
    DefaultLabel(HashMap<String, String>),
    Project(UserProject),
}

/// Exposes the user settings most often used by the CLI
// TODO Unify with API user settings type
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserSettings {
    pub version: u32,
    pub projects: HashMap<String, Setting>,
}

impl UserSettings {
    /// Sets the threshold for the given risk domain.
    pub fn set_threshold(&mut self, project_id: String, name: String, threshold: Threshold) {
        // log::debug!("Retrieving user settings for project: {}", project_id);
        let mut thresholds = self
            .projects
            .get(project_id.as_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| {
                Setting::Project(UserProject {
                    thresholds: HashMap::new(),
                })
            });

        if let Setting::Project(ref mut t) = thresholds {
            t.thresholds.insert(name, threshold);
        }

        self.projects.insert(project_id, thresholds);
    }
}
