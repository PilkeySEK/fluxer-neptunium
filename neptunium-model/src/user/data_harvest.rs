use serde::{Deserialize, Serialize};

#[cfg(feature = "user_api")]
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataHarvestStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct DataHarvestResponse {
    pub harvest_id: String,
    pub status: DataHarvestStatus,
    // ISO8601
    pub created_at: jiff::Timestamp,
    // ISO8601
    /// `None` if the data harvest is pending.
    pub started_at: Option<jiff::Timestamp>,
    // ISO8601
    /// `None` if the data harvest has not completed yet.
    pub completed_at: Option<jiff::Timestamp>,
    // ISO8601
    /// `None` if the data harvest has not failed.
    pub failed_at: Option<jiff::Timestamp>,
    /// File size expressed as a string.
    pub file_size: Option<String>,
    /// 0-100.
    pub progress_percent: u8,
    /// Textual description of the current harvest step, if available.
    pub progress_step: Option<String>,
    /// Error message if the harvest has failed.
    pub error_message: Option<String>,
    // ISO8601
    /// When the download URL expires, or `None` if no download url is available.
    pub download_url_expires_at: Option<jiff::Timestamp>,
    // ISO8601
    /// When the download expires, or `None` if no download is available.
    pub expires_at: Option<jiff::Timestamp>,
}
