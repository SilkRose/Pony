use super::user::UserData;
use super::{ApiDebug, ApiMeta, RelationshipData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogApi {
	pub data: BlogData,
	pub included: Vec<UserData>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogData {
	pub id: String,
	pub r#type: String,
	pub attributes: BlogAttributes,
	pub relationships: BlogRelationships,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogAttributes {
	pub title: String,
	pub date_posted: String,
	pub content: String,
	pub num_views: u32,
	pub num_comments: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogRelationships {
	pub tagged_story: RelationshipData,
}
