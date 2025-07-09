use super::ApiDebug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagApi<T = u32> {
	pub data: Vec<TagData<T>>,
	pub included: Vec<()>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: TagAttributes<T>,
	pub meta: TagMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagAttributes<T = u32> {
	pub name: String,
	pub r#type: String,
	pub num_stories: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagMeta {
	pub old_id: String,
	pub url: String,
}
