use std::fmt;
use std::str::FromStr;

use serde::Deserialize;

use super::errors::aikido::AikidoError;

/// Aikido severity levels
#[derive(
	Debug, Clone, Copy,
	PartialEq, Eq, Hash, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
	Critical,
	High,
	Medium,
	Low,
}

/// Display order: critical first
pub const SEV_ORDER: [Severity; 4] = [
	Severity::Critical,
	Severity::High,
	Severity::Medium,
	Severity::Low,
];

impl fmt::Display for Severity {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			Self::Critical => write!(f, "critical"),
			Self::High => write!(f, "high"),
			Self::Medium => write!(f, "medium"),
			Self::Low => write!(f, "low"),
		}
	}
}

impl FromStr for Severity {
	type Err = AikidoError;

	fn from_str(
		s: &str,
	) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"critical" => Ok(Self::Critical),
			"high" => Ok(Self::High),
			"medium" => Ok(Self::Medium),
			"low" => Ok(Self::Low),
			other => Err(AikidoError::Parse(
				format!(
					"unknown severity: {other}"
				),
			)),
		}
	}
}

/// Serde default: lowest severity
fn default_severity() -> Severity {
	Severity::Low
}

/// Serde default: unknown reachability
fn default_reachability() -> String {
	"unknown".into()
}

/// Security issue from Aikido API
#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
	#[serde(default)]
	pub id: u64,
	#[serde(default, rename = "type")]
	pub issue_type: String,
	#[serde(default = "default_severity")]
	pub severity: Severity,
	#[serde(default)]
	pub severity_score: u8,
	#[serde(default)]
	pub status: String,
	#[serde(default)]
	pub affected_package: String,
	#[serde(default)]
	pub affected_file: String,
	#[serde(default)]
	pub cve_id: String,
	#[serde(default)]
	pub code_repo_name: String,
	#[serde(default)]
	pub container_repo_name: String,
	#[serde(default)]
	pub start_line: Option<u64>,
	#[serde(default)]
	pub installed_version: String,
	#[serde(default)]
	pub patched_versions: Vec<String>,
}

/// Detailed issue with fix info
#[derive(Debug, Clone, Deserialize)]
pub struct IssueDetail {
	#[serde(default)]
	pub id: u64,
	#[serde(default, rename = "type")]
	pub issue_type: String,
	#[serde(default = "default_severity")]
	pub severity: Severity,
	#[serde(default)]
	pub severity_score: u8,
	#[serde(default)]
	pub status: String,
	#[serde(default)]
	pub affected_package: String,
	#[serde(default)]
	pub affected_file: String,
	#[serde(default)]
	pub cve_id: String,
	#[serde(default)]
	pub code_repo_name: String,
	#[serde(default)]
	pub container_repo_name: String,
	#[serde(default)]
	pub how_to_fix: String,
	#[serde(default = "default_reachability")]
	pub reachability_status: String,
	#[serde(default)]
	pub first_detected_at: Option<u64>,
}

/// Code repository monitored by Aikido
#[derive(Debug, Clone, Deserialize)]
pub struct Repo {
	#[serde(default)]
	pub id: u64,
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub provider: String,
}

/// Container image repository
#[derive(Debug, Clone, Deserialize)]
pub struct ContainerRepo {
	#[serde(default)]
	pub id: u64,
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub registry: String,
	#[serde(default)]
	pub image_count: u64,
	#[serde(default)]
	pub active: bool,
	#[serde(default)]
	pub last_scanned_at: Option<u64>,
}

/// Issue count totals by severity
#[derive(Debug, Clone, Deserialize)]
pub struct IssueCounts {
	#[serde(default)]
	pub issues: SeverityCounts,
}

/// Per-severity counters
#[derive(
	Debug, Clone, Default, Deserialize,
)]
pub struct SeverityCounts {
	#[serde(default)]
	pub critical: u64,
	#[serde(default)]
	pub high: u64,
	#[serde(default)]
	pub medium: u64,
	#[serde(default)]
	pub low: u64,
	#[serde(default)]
	pub all: u64,
}

/// Filters for issue listing
#[derive(Debug, Default, Clone)]
pub struct IssueFilters {
	pub repo_name: Option<String>,
	pub container_name: Option<String>,
	pub issue_type: Option<String>,
	pub severities: Option<Vec<String>>,
}

/// Filters for issue count query
#[derive(Debug, Default, Clone)]
pub struct IssueCountFilters {
	pub repo_name: Option<String>,
	pub container_repo_id: Option<u64>,
}
