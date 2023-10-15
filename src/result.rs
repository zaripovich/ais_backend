use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct MResult<T> {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}
