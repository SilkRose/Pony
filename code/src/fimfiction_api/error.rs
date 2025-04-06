use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FimficError<T = u32> {
	errors: Vec<FimficErrorInner<T>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FimficErrorInner<T = u32> {
	pub status: String,
	pub title: String,
	pub detail: Option<String>,
	pub code: T,
	pub meta: Option<ErrorMeta<T>>,
	pub links: ErrorLinks,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorMeta<T = u32> {
	pub resource: String,
	pub id: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorLinks {
	about: String,
}
