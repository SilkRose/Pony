use super::{ApiDebug, ApiMeta, RelationshipData};
use crate::fimfiction_api::ApiIncluded;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadApi<T = u32> {
	pub data: Vec<ThreadData<T>>,
	pub included: Vec<ApiIncluded<T>>,
	pub links: ThreadLinks,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: ThreadAttributes<T>,
	pub relationships: ThreadRelationships,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadAttributes<T = u32> {
	pub title: String,
	pub num_posts: T,
	pub date_created: String,
	pub date_last_post: String,
	pub sticky: bool,
	pub locked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadRelationships {
	pub creator: RelationshipData,
	pub group: RelationshipData,
	pub last_poster: RelationshipData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadLinks {
	pub first: String,
	pub last: String,
}
