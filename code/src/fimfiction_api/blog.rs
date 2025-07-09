use super::{ApiDebug, ApiMeta, RelationshipData};
use crate::fimfiction_api::ApiIncluded;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogApi<T = u32> {
	pub data: BlogData<T>,
	pub included: Vec<ApiIncluded<T>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: BlogAttributes<T>,
	pub relationships: BlogRelationships,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogAttributes<T = u32> {
	pub title: String,
	pub date_posted: String,
	pub content: String,
	pub num_views: T,
	pub num_comments: T,
	pub site_post: bool,
	pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogRelationships {
	pub tagged_story: RelationshipData,
}
