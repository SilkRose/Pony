use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryApi {
	pub data: ApiData,
	pub included: Vec<ApiIncluded>,
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
	pub links: DataLinks,
	pub meta: DataMeta,
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
pub struct AttributesColor {
	pub hex: String,
	pub rgb: (u32, u32, u32),
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
pub struct DataLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataMeta {
	pub url: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum ApiIncluded {
	Tag(IncludedTag),
	Author(IncludedAuthor),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncludedTag {
	pub id: String,
	pub r#type: String,
	pub attributes: TagAttributes,
	pub meta: TagMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagAttributes {
	pub name: String,
	pub r#type: String,
	pub num_stories: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagMeta {
	pub old_id: String,
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncludedAuthor {
	pub id: String,
	pub r#type: String,
	pub attributes: AuthorAttributes,
	pub links: AuthorLinks,
	pub meta: AuthorMeta,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesAvatar {
	#[serde(rename = "32")]
	pub r32: String,
	#[serde(rename = "48")]
	pub r48: String,
	#[serde(rename = "64")]
	pub r64: String,
	#[serde(rename = "96")]
	pub r96: String,
	#[serde(rename = "128")]
	pub r128: String,
	#[serde(rename = "160")]
	pub r160: String,
	#[serde(rename = "192")]
	pub r192: String,
	#[serde(rename = "256")]
	pub r256: String,
	#[serde(rename = "320")]
	pub r320: String,
	#[serde(rename = "384")]
	pub r384: String,
	#[serde(rename = "512")]
	pub r512: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorMeta {
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct APILinks {
	pub first: String,
	pub prev: String,
	pub next: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct APIMeta {
	pub num_stories: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiDebug {
	pub duration: String,
}
