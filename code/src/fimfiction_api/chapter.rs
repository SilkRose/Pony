use super::{ApiDebug, ApiLinks, ApiMeta, RelationshipData};
use crate::fimfiction_api::ApiIncluded;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterApi<T = u32> {
	pub data: ChapterData<T>,
	pub included: Vec<ApiIncluded<T>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: ChapterAttributes<T>,
	pub relationships: ChapterRelationship,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterRelationship {
	story: RelationshipData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChapterAttributes<T = u32> {
	pub chapter_number: T,
	pub title: String,
	pub published: bool,
	pub num_words: T,
	pub num_views: T,
	pub date_published: String,
	pub date_modified: String,
	pub authors_note_position: String,
}
