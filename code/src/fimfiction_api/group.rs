use super::{ApiDebug, ApiLinks, ApiMeta, RelationshipData};
use crate::fimfiction_api::ApiIncluded;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupApi<T = u32> {
	pub data: GroupData<T>,
	pub included: Vec<ApiIncluded<T>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: GroupAttributes<T>,
	pub relationships: GroupRelationship,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupRelationship {
	pub founder: RelationshipData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupAttributes<T = u32> {
	pub name: String,
	pub num_members: T,
	pub num_stories: T,
	pub icon: GroupIcon,
	pub nsfw: bool,
	pub open: bool,
	pub hidden: bool,
	pub date_created: String,
	pub description: String,
	pub description_html: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupIcon {
	#[serde(rename = "32")]
	pub r32: Option<String>,
	#[serde(rename = "48")]
	pub r48: Option<String>,
	#[serde(rename = "64")]
	pub r64: Option<String>,
	#[serde(rename = "96")]
	pub r96: Option<String>,
	#[serde(rename = "128")]
	pub r128: Option<String>,
	#[serde(rename = "160")]
	pub r160: Option<String>,
	#[serde(rename = "192")]
	pub r192: Option<String>,
	#[serde(rename = "256")]
	pub r256: Option<String>,
	#[serde(rename = "320")]
	pub r320: Option<String>,
	#[serde(rename = "384")]
	pub r384: Option<String>,
	#[serde(rename = "512")]
	pub r512: Option<String>,
}
