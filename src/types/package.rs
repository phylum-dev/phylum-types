//! Module containing data types reprsenting on-the-wire data for packages

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::common::*;

/// The package ecosystem
// TODO Should be Ecosystem?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Npm,
    Python,
    Java,
    Ruby,
}

/// Human friendly risk level buckets
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum RiskLevel {
    /// Informational, no action needs to be taken
    #[serde(rename = "info")]
    Info,
    /// Minor issues like cosmetic code smells
    #[serde(rename = "low")]
    /// Possibly a problem in great number or rare circumstances
    Low,
    /// May be indicative of overall quality issues
    #[serde(rename = "medium")]
    Medium,
    /// Possibly exploitable behavior in some circumstances
    #[serde(rename = "high")]
    High,
    /// Should fix as soon as possible, may be under active exploitation
    #[serde(rename = "critical")]
    Critical,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let risk_level = format!("{:?}", self);
        write!(f, "{}", risk_level.to_lowercase())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
// TODO Naming here seems inconsisten, some have Risk as a suffix, others don't
pub enum RiskDomain {
    /// Malicious code such as malware or crypto miners
    #[serde(rename = "malicious_code")]
    MaliciousCode,
    /// A code vulnerability such as use-after-free or other code smell
    #[serde(rename = "vulnerability")]
    Vulnerabilities,
    /// Poor engineering practices and other code smells
    #[serde(rename = "engineering")]
    EngineeringRisk,
    /// One or more authors is a possible bad actor or other problems
    #[serde(rename = "author")]
    AuthorRisk,
    /// License is unknown, incompatible with the project, etc
    #[serde(rename = "license")]
    LicenseRisk,
}

impl fmt::Display for RiskDomain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO for a human readable format, do we gain anything with this?
        let risk_domain = match self {
            RiskDomain::MaliciousCode => "MAL",
            RiskDomain::Vulnerabilities => "VLN",
            RiskDomain::EngineeringRisk => "ENG",
            RiskDomain::AuthorRisk => "AUT",
            RiskDomain::LicenseRisk => "LIC",
        };
        write!(f, "{}", risk_domain)
    }
}

/// Represents an actionable issue found in a package
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    /// Name of issue
    pub title: String,
    /// Description of Issue and possible remediation steps
    pub description: String,
    #[serde(alias = "severity")]
    /// How risky is it
    pub risk_level: RiskLevel,
    /// The domain of the risk
    #[serde(alias = "domain")]
    pub risk_domain: RiskDomain,
}

/// The results of an individual heuristic run
#[derive(Debug, Deserialize, Serialize)]
pub struct HeuristicResult {
    /// The risk domain
    pub domain: RiskDomain,
    /// The score
    pub score: f64,
    /// The risk level bucket it falls into
    pub risk_level: RiskLevel,
}

/// A vulnerability
#[derive(Debug, Deserialize, Serialize)]
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

impl FromStr for PackageType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "npm" => Ok(Self::Npm),
            "python" => Ok(Self::Python),
            "java" => Ok(Self::Java),
            "ruby" => Ok(Self::Ruby),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let package_type = format!("{:?}", self);
        write!(f, "{}", package_type.to_lowercase())
    }
}

impl PackageType {
    pub fn language(&self) -> &str {
        match self {
            PackageType::Npm => "Javascript",
            PackageType::Ruby => "Ruby",
            PackageType::Python => "Python",
            PackageType::Java => "Java",
        }
    }
}

/// Describes a package in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageDescriptor {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub package_type: PackageType,
}

/// Basic core package meta data
// TODO Clearer name
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
