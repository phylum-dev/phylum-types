//! Module containing data types reprsenting on-the-wire data for packages

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::common::Status;

/// Risk domains.
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[repr(u8)]
pub enum RiskDomain {
    /// One or more authors is a possible bad actor or other problems
    #[serde(rename = "author")]
    AuthorRisk = 0,
    /// Poor engineering practices and other code smells
    #[serde(rename = "engineering")]
    EngineeringRisk = 1,
    /// Malicious code such as malware or crypto miners
    #[serde(rename = "malicious_code")]
    #[serde(alias = "malicious")]
    Malicious = 2,
    /// A code vulnerability such as use-after-free or other code smell
    #[serde(rename = "vulnerability")]
    Vulnerabilities = 3,
    /// License is unknown, incompatible with the project, etc
    #[serde(rename = "license")]
    LicenseRisk = 4,
}

impl fmt::Display for RiskDomain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        RiskType::from(*self).fmt(f)
    }
}

/// Issue severity.
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel {
    /// Informational, no action needs to be taken.
    Info,
    /// Minor issues like cosmetic code smells,
    /// possibly a problem in great number or rare circumstances.
    Low,
    /// May be indicative of overall quality issues.
    Medium,
    /// Possibly exploitable behavior in some circumstances.
    High,
    /// Should fix as soon as possible, may be under active exploitation.
    Critical,
}

impl RiskLevel {
    pub fn score(&self) -> f32 {
        match self {
            RiskLevel::Info => 1.,
            RiskLevel::Low => 0.8,
            RiskLevel::Medium => 0.65,
            RiskLevel::High => 0.35,
            RiskLevel::Critical => 0.1,
        }
    }
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let risk_level = format!("{self:?}");
        write!(f, "{}", risk_level.to_lowercase())
    }
}

/// The package ecosystem
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Npm,
    PyPi,
    Maven,
    RubyGems,
    Nuget,
    Cargo,
    Golang,
}

impl PackageType {
    pub fn language(&self) -> &str {
        match self {
            PackageType::Npm => "Javascript",
            PackageType::RubyGems => "Ruby",
            PackageType::PyPi => "Python",
            PackageType::Maven => "Java",
            PackageType::Nuget => ".NET",
            PackageType::Cargo => "Rust",
            PackageType::Golang => "Golang",
        }
    }
}

impl FromStr for PackageType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "npm" => Ok(Self::Npm),
            "python" | "pypi" => Ok(Self::PyPi),
            "maven" => Ok(Self::Maven),
            "ruby" | "rubygems" => Ok(Self::RubyGems),
            "nuget" => Ok(Self::Nuget),
            "cargo" => Ok(Self::Cargo),
            "golang" => Ok(Self::Golang),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let package_type = format!("{self:?}");
        write!(f, "{}", package_type.to_lowercase())
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ScoredVersion {
    pub version: String,
    pub total_risk_score: Option<f32>,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct PackageSpecifier {
    #[serde(alias = "type")]
    pub registry: String,
    pub name: String,
    pub version: String,
}

// TODO Once we unify PackageDescriptor and PackageSpecifier, this goes away
impl From<&PackageDescriptor> for PackageSpecifier {
    fn from(descriptor: &PackageDescriptor) -> Self {
        Self {
            registry: descriptor.package_type.to_string(),
            name: descriptor.name.clone(),
            version: descriptor.version.clone(),
        }
    }
}

impl TryFrom<PackageSpecifier> for PackageDescriptor {
    type Error = String;

    fn try_from(value: PackageSpecifier) -> Result<Self, Self::Error> {
        let PackageSpecifier {
            registry,
            name,
            version,
        } = value;
        let package_type = PackageType::from_str(&registry)
            .map_err(|()| format!("Failed to convert registry {registry} to package type"))?;
        Ok(PackageDescriptor {
            name,
            version,
            package_type,
        })
    }
}

/// Risk scores by domain.
#[derive(
    PartialEq, PartialOrd, Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema,
)]
pub struct RiskScores {
    pub total: f32,
    pub vulnerability: f32,
    #[serde(rename = "malicious_code")]
    #[serde(alias = "malicious")]
    pub malicious: f32,
    pub author: f32,
    pub engineering: f32,
    pub license: f32,
}

/// Change in score over time.
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScoreDynamicsPoint {
    pub date_time: DateTime<Utc>,
    pub score: f32,
    pub label: String,
}

/// A single package issue.
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct Issue {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    #[serde(alias = "risk_level")]
    pub severity: RiskLevel,
    #[serde(alias = "risk_domain")]
    pub domain: RiskDomain,
}

/// Issue description.
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssuesListItem {
    pub risk_type: RiskType,
    pub score: f32,
    pub impact: RiskLevel,
    pub description: String,
    pub title: String,
    pub tag: Option<String>,
    pub id: Option<String>,
    pub ignored: IgnoredReason,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum RiskType {
    TotalRisk,
    Vulnerabilities,
    #[serde(alias = "maliciousRisk")]
    #[serde(rename = "maliciousCodeRisk")]
    MaliciousRisk,
    AuthorsRisk,
    EngineeringRisk,
    LicenseRisk,
}

impl From<RiskDomain> for RiskType {
    fn from(risk_domain: RiskDomain) -> Self {
        match risk_domain {
            RiskDomain::Malicious => RiskType::MaliciousRisk,
            RiskDomain::Vulnerabilities => RiskType::Vulnerabilities,
            RiskDomain::EngineeringRisk => RiskType::EngineeringRisk,
            RiskDomain::AuthorRisk => RiskType::AuthorsRisk,
            RiskDomain::LicenseRisk => RiskType::LicenseRisk,
        }
    }
}

impl fmt::Display for RiskType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let risk_domain = match self {
            RiskType::MaliciousRisk => "MAL",
            RiskType::Vulnerabilities => "VLN",
            RiskType::EngineeringRisk => "ENG",
            RiskType::AuthorsRisk => "AUT",
            RiskType::LicenseRisk => "LIC",
            RiskType::TotalRisk => "ALL",
        };
        write!(f, "{risk_domain}")
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum IgnoredReason {
    False,
    FalsePositive,
    NotRelevant,
    Other,
}

/// Author information
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub avatar_url: String,
    pub email: String,
    pub profile_url: String,
}

/// Responsiveness of developers
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct DeveloperResponsiveness {
    pub open_issue_count: Option<usize>,
    pub total_issue_count: Option<usize>,
    pub open_issue_avg_duration: Option<u32>,
    pub open_pull_request_count: Option<usize>,
    pub total_pull_request_count: Option<usize>,
    pub open_pull_request_avg_duration: Option<u32>,
}

/// Count of issues for each severity.
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IssueImpacts {
    pub low: u32,
    pub medium: u32,
    pub high: u32,
    pub critical: u32,
}

impl From<&[Issue]> for IssueImpacts {
    fn from(issues: &[Issue]) -> Self {
        let mut impacts = IssueImpacts::default();
        for issue in issues {
            match issue.severity.score() {
                score if score >= 0.8 => impacts.low += 1,
                score if (0.5..0.8).contains(&score) => impacts.medium += 1,
                score if (0.2..0.5).contains(&score) => impacts.high += 1,
                _ => impacts.critical += 1,
            }
        }
        impacts
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "status", content = "data")]
pub enum PackageSubmitResponse {
    AlreadyProcessed(Package),
    AlreadySubmitted,
    New,
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub registry: String,
    pub published_date: Option<String>,
    pub latest_version: Option<String>,
    pub versions: Vec<ScoredVersion>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub dep_specs: Vec<PackageSpecifier>,
    pub dependencies: Option<Vec<Package>>,
    pub download_count: u32,
    pub risk_scores: RiskScores,
    pub total_risk_score_dynamics: Option<Vec<ScoreDynamicsPoint>>,
    pub issues_details: Vec<Issue>,
    pub issues: Vec<IssuesListItem>,
    pub authors: Vec<Author>,
    pub developer_responsiveness: Option<DeveloperResponsiveness>,
    pub issue_impacts: IssueImpacts,
    pub complete: bool,
    pub release_data: Option<PackageReleaseData>,
    pub repo_url: Option<String>,
    #[serde(rename = "maintainers_recently_changed")]
    pub maintainers_recently_changed: Option<bool>,
    #[serde(rename = "is_abandonware")]
    pub is_abandonware: Option<bool>,
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Default, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct PackageReleaseData {
    pub first_release_date: String,
    pub last_release_date: String,
}

// v--- TODO: OLD PACKAGE RESPONSES ---v //

/// The results of an individual heuristic run
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeuristicResult {
    /// The risk domain
    pub domain: RiskDomain,
    /// The score
    pub score: f64,
    /// The risk level bucket it falls into
    pub risk_level: RiskLevel,
}

/// A vulnerability
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Vulnerability {
    /// If this vulnerability falls into one or more known CVEs
    pub cve: Vec<String>,
    /// Severity of the vulnerability
    #[serde(rename = "severity")]
    pub base_severity: f32,
    /// What risk level bucket it falls into
    pub risk_level: RiskLevel,
    /// Title of the vulnerability
    pub title: String,
    /// A more in depth description
    pub description: String,
    /// Remediation information if known
    pub remediation: String,
}

/// Describes a package in the system
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct PackageDescriptor {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    #[serde(alias = "registry")]
    pub package_type: PackageType,
}

/// Basic core package meta data
// TODO Clearer name
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageStatus {
    /// Name of the package
    pub name: String,
    /// Package version
    pub version: String,
    /// Package processing status
    // TODO Better name, such as processing_status?
    pub status: Status,
    /// Last updates, as epoch seconds
    pub last_updated: u64,
    /// Package license
    pub license: Option<String>,
    /// The overall quality score of the package
    pub package_score: Option<f64>,
    /// Number of dependencies
    // TODO Break out by type? dev / optional / core?
    pub num_dependencies: u32,
    /// Number of vulnerabilities found in this package and all transitive
    /// dependencies
    pub num_vulnerabilities: u32,
}

/// Package metadata with extended info info
// TODO Clearer name
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageStatusExtended {
    #[serde(flatten)]
    pub basic_status: PackageStatus,
    /// The package_type, npm, etc.
    // TODO Replace with ecosystem?
    #[serde(rename = "type")]
    pub package_type: PackageType,
    // TODO This might a leftover of the api work going as we eliminate / merge some services, some
    // of which had inconsistent naming styles
    #[serde(rename = "riskVectors")]
    pub risk_vectors: HashMap<String, f64>,
    /// Dependencies of this package
    pub dependencies: HashMap<String, String>,
    /// Any issues found that may need action, but aren't in and of themselves
    /// vulnerabilities
    pub issues: Vec<Issue>,
}
