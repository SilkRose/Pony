use serde::{Deserialize, Serialize};

pub mod blog;
pub mod story;
pub mod user;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiMeta {
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiDebug {
	pub duration: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesColor {
	pub hex: String,
	pub rgb: (u32, u32, u32),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipDataVec {
	pub data: Vec<DataType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationshipData {
	pub data: DataType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataType {
	pub r#type: String,
	pub id: String,
}
