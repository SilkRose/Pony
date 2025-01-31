use super::user::{AttributesAvatar, UserData};
use super::{ApiDebug, ApiLinks, ApiMeta, AttributesColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryApi {
	pub data: ApiData,
	pub included: Vec<UserData<AuthorAttributes>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiData {
	pub id: String,
	pub r#type: String,
	pub attributes: DataAttributes,
	pub relationships: DataRelationships,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataAttributes {
	pub title: String,
	pub short_description: String,
	pub description: String,
	pub description_html: String,
	pub date_modified: String,
	pub date_updated: String,
	pub date_published: String,
	pub published: bool,
	pub cover_image: Option<AttributesCoverImage>,
	pub color: AttributesColor,
	pub num_views: u32,
	pub total_num_views: u32,
	pub num_words: u32,
	pub num_chapters: u32,
	pub num_comments: u32,
	pub rating: u32,
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
pub struct DataRelationships {
	pub author: RelationshipAuthor,
	pub tags: RelationshipTags,
	pub prequel: Option<RelationshipPrequel>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipAuthor {
	pub data: AuthorData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipTags {
	pub data: Vec<TagData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipPrequel {
	pub data: PrequelData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrequelData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorAttributes {
	pub name: String,
	pub bio: String,
	pub bio_html: String,
	pub num_followers: u32,
	pub num_stories: u32,
	pub num_blog_posts: u32,
	pub avatar: AttributesAvatar,
	pub color: AttributesColor,
	pub date_joined: String,
}
