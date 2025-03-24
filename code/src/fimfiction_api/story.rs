use super::user::UserData;
use super::{ApiDebug, ApiLinks, ApiMeta, AttributesColor, RelationshipData, RelationshipDataVec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryApi<T = u32> {
	pub data: StoryData<T>,
	pub included: Vec<UserData<T>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: StoryAttributes<T>,
	pub relationships: StoryRelationships,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryAttributes<T = u32> {
	pub title: String,
	pub short_description: String,
	pub description: String,
	pub description_html: String,
	pub date_modified: String,
	pub date_updated: Option<String>,
	pub date_published: Option<String>,
	pub published: bool,
	pub cover_image: Option<AttributesCoverImage>,
	pub color: AttributesColor,
	pub num_views: T,
	pub total_num_views: T,
	pub num_words: T,
	pub num_chapters: T,
	pub num_comments: T,
	pub rating: T,
	pub status: String,
	pub submitted: bool,
	pub completion_status: String,
	pub content_rating: String,
	pub num_likes: i32,
	pub num_dislikes: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesCoverImage {
	pub thumbnail: String,
	pub medium: String,
	pub large: String,
	pub full: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryRelationships {
	pub author: RelationshipData,
	pub tags: RelationshipDataVec,
	pub prequel: Option<RelationshipData>,
}
