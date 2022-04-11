// This is a reference for the Maven project descriptor used in Maven.
// https://maven.apache.org/ref/3.8.4/maven-model/maven.html

use std::collections::HashMap;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String>,
    pub parent: Option<Parent>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<String>,
    pub packaging: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "inceptionYear")]
    pub inception_year: Option<String>,
    pub organization: Option<Organization>,
    pub licenses: Option<Licenses>,
    pub developers: Option<Developers>,
    pub contributors: Option<Contributors>,
    #[serde(rename = "mailingLists")]
    pub mailing_lists: Option<MailingLists>,
    pub prerequisites: Option<Prerequisites>,
    pub modules: Option<Modules>,
    pub scm: Option<Scm>,
    #[serde(rename = "issueManagement")]
    pub issue_management: Option<IssueManagement>,
    #[serde(rename = "ciManagement")]
    pub ci_management: Option<CiManagement>,
    #[serde(rename = "distributionManagement")]
    pub distribution_management: Option<DistributionManagement>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,
    pub dependencies: Option<Dependencies>,
    pub repositories: Option<Repositories>,
    #[serde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,
    pub build: Option<Build>,
    pub reports: Option<Reports>,
    pub reporting: Option<Reporting>,
    pub profiles: Option<Profiles>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Licenses {
    #[serde(rename = "license", default)]
    pub licenses: Vec<License>,
}

impl Deref for Licenses {
    type Target = Vec<License>;

    fn deref(&self) -> &Self::Target {
        &self.licenses
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
    pub distribution: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Organization {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Developers {
    #[serde(rename = "developer", default)]
    pub developers: Vec<Person>,
}

impl Deref for Developers {
    type Target = Vec<Person>;

    fn deref(&self) -> &Self::Target {
        &self.developers
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Contributors {
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Person>,
}

impl Deref for Contributors {
    type Target = Vec<Person>;

    fn deref(&self) -> &Self::Target {
        &self.contributors
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MailingLists {
    #[serde(rename = "mailingList", default)]
    pub mailing_list: Vec<MailingList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MailingList {
    pub name: Option<String>,
    pub subscribe: Option<String>,
    pub unsubscribe: Option<String>,
    pub post: Option<String>,
    pub archive: Option<String>,
    #[serde(rename = "otherArchives", default)]
    pub other_archives: Vec<OtherArchive>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OtherArchive {
    #[serde(rename = "otherArchive", default)]
    pub other_archive: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Prerequisites {
    pub maven: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Person {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "organizationUrl")]
    pub organization_url: Option<String>,
    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Roles {
    #[serde(rename = "role", default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Parent {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "relativePath")]
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Dependencies {
    #[serde(rename = "dependency", default)]
    pub dependencies: Vec<Dependency>,
}

impl Deref for Dependencies {
    type Target = Vec<Dependency>;

    fn deref(&self) -> &Self::Target {
        &self.dependencies
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub dtype: Option<String>,
    pub classifier: Option<String>,
    pub scope: Option<String>,
    #[serde(rename = "systemPath")]
    pub system_path: Option<String>,
    pub exclusions: Option<Exclusions>,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Exclusions {
    #[serde(rename = "exclusion", default)]
    pub exclusions: Vec<Exclusion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Exclusion {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Modules {
    #[serde(rename = "module", default)]
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Build {
    #[serde(rename = "sourceDirectory")]
    pub source_directory: Option<String>,
    #[serde(rename = "scriptSourceDirectory")]
    pub script_source_directory: Option<String>,
    #[serde(rename = "testSourceDirectory")]
    pub test_source_directory: Option<String>,
    #[serde(rename = "outputDirectory")]
    pub output_directory: Option<String>,
    #[serde(rename = "testOutputDirectory")]
    pub test_output_directory: Option<String>,
    pub extensions: Option<Extensions>,
    #[serde(rename = "defaultGoal")]
    pub default_goal: Option<String>,
    pub resources: Option<Resources>,
    #[serde(rename = "testResources")]
    pub test_resources: Option<TestResources>,
    pub directory: Option<String>,
    #[serde(rename = "finalName")]
    pub final_name: Option<String>,
    pub filters: Option<Filters>,
    #[serde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,
    pub plugins: Option<Plugins>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Filters {
    #[serde(rename = "filter", default)]
    pub filters: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Resources {
    #[serde(rename = "resource", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TestResources {
    #[serde(rename = "testResource", default)]
    pub test_resources: Vec<Resource>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    #[serde(rename = "targetPath")]
    pub target_path: Option<String>,
    pub filtering: Option<bool>,
    pub directory: Option<String>,
    pub includes: Option<Includes>,
    pub excludes: Option<Excludes>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Includes {
    #[serde(rename = "include", default)]
    pub includes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Excludes {
    #[serde(rename = "exclude", default)]
    pub excludes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Plugins {
    #[serde(rename = "plugin", default)]
    pub plugins: Vec<Plugin>,
}

impl Deref for Plugins {
    type Target = Vec<Plugin>;

    fn deref(&self) -> &Self::Target {
        &self.plugins
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PluginManagement {
    pub plugins: Plugins,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Plugin {
    #[serde(rename = "groupId", default = "default_plugin_group_id")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub extensions: Option<String>,
    pub executions: Option<Executions>,
    pub dependencies: Option<Dependencies>,
    pub goals: Option<Goals>,
    pub inherited: Option<bool>,
    pub configuration: Option<Configuration>,
}

fn default_plugin_group_id() -> Option<String> {
    Some(String::from("org.apache.maven.plugins"))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Executions {
    #[serde(rename = "execution", default)]
    pub executions: Vec<Execution>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Execution {
    pub id: Option<String>,
    pub phase: Option<String>,
    pub goals: Option<Goals>,
    pub inherited: Option<bool>,
    pub configuration: Option<Configuration>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Configuration {
    // empty because this is different for every plugin and execution :(
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Reports {
    #[serde(rename = "report", default)]
    pub reports: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Goals {
    #[serde(rename = "goal", default)]
    pub goals: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Extensions {
    #[serde(rename = "extension", default)]
    pub extensions: Vec<Extension>,
}

impl Deref for Extensions {
    type Target = Vec<Extension>;

    fn deref(&self) -> &Self::Target {
        &self.extensions
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Extension {
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Profiles {
    #[serde(rename = "profile", default)]
    pub profiles: Vec<Profile>,
}

impl Deref for Profiles {
    type Target = Vec<Profile>;

    fn deref(&self) -> &Self::Target {
        &self.profiles
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub id: Option<String>,
    pub activation: Option<Activation>,
    pub build: Option<Build>,
    pub modules: Option<Modules>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,
    pub dependencies: Option<Dependencies>,
    pub repositories: Option<Repositories>,
    #[serde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,
    pub reports: Option<Reports>,
    pub reporting: Option<Reporting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Activation {
    #[serde(rename = "activeByDefault")]
    pub active_by_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DependencyManagement {
    pub dependencies: Dependencies,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Repositories {
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PluginRepositories {
    #[serde(rename = "pluginRepository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    #[serde(rename = "uniqueVersion")]
    pub unique_version: Option<bool>,
    pub releases: Option<RepositoryPolicy>,
    pub snapshots: Option<RepositoryPolicy>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RepositoryPolicy {
    pub enabled: Option<String>,
    #[serde(rename = "updatePolicy")]
    pub update_policy: Option<String>,
    #[serde(rename = "checksumPolicy")]
    pub checksum_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Scm {
    pub connection: Option<String>,
    #[serde(rename = "developerConnection")]
    pub developer_connection: Option<String>,
    pub tag: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct IssueManagement {
    pub system: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CiManagement {
    pub system: Option<String>,
    pub url: Option<String>,
    pub notifiers: Vec<Notifier>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Notifier {
    #[serde(rename = "type")]
    pub ntype: Option<String>,
    #[serde(rename = "sendOnError")]
    pub send_on_error: Option<bool>,
    #[serde(rename = "sendOnFailure")]
    pub send_on_failure: Option<bool>,
    #[serde(rename = "sendOnSuccess")]
    pub send_on_success: Option<bool>,
    #[serde(rename = "sendOnWarning")]
    pub send_on_warning: Option<bool>,
    pub address: String,
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DistributionManagement {
    pub repository: Option<Repository>,
    #[serde(rename = "snapshotRepository")]
    pub snapshot_repository: Option<Repository>,
    pub site: Option<String>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    pub relocation: Option<Relocation>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Relocation {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Reporting {
    #[serde(rename = "excludeDefaults")]
    pub exclude_defaults: Option<String>,
    #[serde(rename = "outputDirectory")]
    pub output_directory: Option<String>,
    pub plugins: Option<Plugins>,
}
