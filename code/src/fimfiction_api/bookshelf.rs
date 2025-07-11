use super::{ApiDebug, ApiLinks, ApiMeta, RelationshipData};
use crate::fimfiction_api::ApiIncluded;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookshelfApi<T = u32> {
	pub data: BookshelfData<T>,
	pub included: Vec<ApiIncluded<T>>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookshelfData<T = u32> {
	pub id: String,
	pub r#type: String,
	pub attributes: BookshelfAttributes<T>,
	pub relationships: Option<BookshelfRelationship>,
	pub links: ApiLinks,
	pub meta: ApiMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookshelfAttributes<T = u32> {
	pub name: String,
	pub privacy: String,
	pub description: String,
	pub color: String,
	pub icon: BookshelfIcon,
	pub num_stories: T,
	pub num_unread: Option<T>,
	pub track_unread: bool,
	pub quick_add: bool,
	pub email_on_update: bool,
	pub date_created: String,
	pub date_modified: String,
	pub order: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookshelfIcon {
	pub name: String,
	pub r#type: String,
	pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookshelfRelationship {
	user: RelationshipData,
}
