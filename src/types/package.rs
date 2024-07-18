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
            "maven" | "maven-central" => Ok(Self::Maven),
            "ruby" | "rubygems" | "gem" => Ok(Self::RubyGems),
            "nuget" | "dotnet" => Ok(Self::Nuget),
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

impl From<PackageType> for purl::PackageType {
    fn from(package_type: PackageType) -> purl::PackageType {
        match package_type {
            PackageType::Npm => purl::PackageType::Npm,
            PackageType::PyPi => purl::PackageType::PyPI,
            PackageType::Maven => purl::PackageType::Maven,
            PackageType::RubyGems => purl::PackageType::Gem,
            PackageType::Nuget => purl::PackageType::NuGet,
            PackageType::Cargo => purl::PackageType::Cargo,
            PackageType::Golang => purl::PackageType::Golang,
        }
    }
}

impl TryFrom<purl::PackageType> for PackageType {
    type Error = purl::UnsupportedPackageType;

    fn try_from(
        package_type: purl::PackageType,
    ) -> Result<PackageType, purl::UnsupportedPackageType> {
        Ok(match package_type {
            purl::PackageType::Cargo => PackageType::Cargo,
            purl::PackageType::Gem => PackageType::RubyGems,
            purl::PackageType::Golang => PackageType::Golang,
            purl::PackageType::Maven => PackageType::Maven,
            purl::PackageType::Npm => PackageType::Npm,
            purl::PackageType::NuGet => PackageType::Nuget,
            purl::PackageType::PyPI => PackageType::PyPi,
            _ => return Err(purl::UnsupportedPackageType),
        })
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
    #[serde(skip)]
    pub rule: Option<String>,
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
    pub ignored: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
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
    pub complete: bool,
    pub release_data: Option<PackageReleaseData>,
    pub repo_url: Option<String>,
    pub maintainers_recently_changed: Option<bool>,
    pub is_abandonware: Option<bool>,
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Default, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

/// `PackageDescriptorAndLockfile` represents a parsed package
/// (`package_descriptor`) and the optional path to its lockfile (`lockfile`).
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct PackageDescriptorAndLockfile {
    #[serde(flatten)]
    pub package_descriptor: PackageDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockfile: Option<String>,
}

impl From<&PackageDescriptor> for PackageDescriptorAndLockfile {
    fn from(value: &PackageDescriptor) -> Self {
        PackageDescriptorAndLockfile {
            package_descriptor: value.clone(),
            lockfile: None,
        }
    }
}

impl From<PackageDescriptor> for PackageDescriptorAndLockfile {
    fn from(package_descriptor: PackageDescriptor) -> Self {
        Self {
            package_descriptor,
            lockfile: None,
        }
    }
}

/// `PackageSpecifierAndLockfile` represents a parsed package
/// (`package_specifier`) and the optional path to its lockfile (`lockfile`).
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct PackageSpecifierAndLockfile {
    pub package_specifier: PackageSpecifier,
    pub lockfile: Option<String>,
}

impl From<&PackageSpecifier> for PackageSpecifierAndLockfile {
    fn from(value: &PackageSpecifier) -> Self {
        PackageSpecifierAndLockfile {
            package_specifier: value.clone(),
            lockfile: None,
        }
    }
}

/// `PackageUrlAndLockfile` represents a parsed package (`purl`)
/// and the optional path to its lockfile (`lockfile`).
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize, JsonSchema,
)]
pub struct PackageUrlAndLockfile {
    pub purl: String,
    pub lockfile: Option<String>,
}

/// Basic core package meta data
// TODO Clearer name
#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PackageStatus {
    /// A PURL referencing this package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_vulnerabilities: Option<u32>,
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
    pub issues: Vec<IssueStatus>,
}

/// A dependency issue with its job status.
#[derive(PartialEq, Clone, Debug, Deserialize, Eq, JsonSchema, Serialize)]
pub struct IssueStatus {
    /// The issue.
    #[serde(flatten)]
    pub issue: Issue,
    /// The reason why the issue is ignored (if applicable).
    #[serde(default)]
    pub ignored: Option<String>,
}
