use super::{ApiDebug, ApiLinks, ApiMeta, AttributesColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserApi<T = u32> {
	pub data: UserData<T>,
	pub included: Vec<()>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: UserAttributes<T>,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserAttributes<T = u32> {
	pub name: String,
	pub bio: String,
	pub bio_html: String,
	pub num_followers: T,
	pub num_stories: T,
	pub num_blog_posts: T,
	pub avatar: AttributesAvatar,
	pub date_last_online: Option<String>,
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
