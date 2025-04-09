use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub mod blog;
pub mod chapter;
pub mod error;
pub mod story;
pub mod user;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiMeta {
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiDebug {
	pub duration: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesColor {
	pub hex: String,
	pub rgb: (u8, u8, u8),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipDataVec {
	pub data: Vec<DataType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipData {
	pub data: DataType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataType {
	pub r#type: String,
	pub id: String,
}

pub fn fimfic_api_headers(
	user_agent: Option<&str>, token: &str,
) -> Result<HeaderMap, Box<dyn Error>> {
	let mut headers = HeaderMap::new();
	if let Some(user_agent) = user_agent {
		headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
	}
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(
		CONTENT_TYPE,
		HeaderValue::from_static("application/vnd.api+json"),
	);
	Ok(headers)
}
