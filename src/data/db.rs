use std::borrow::Cow;

use serde::{Deserialize, Serialize, de::Error};

#[derive(Debug, Eq, PartialEq, sqlx::Type, Default)]
#[sqlx(transparent)]
pub struct Uuid(uuid::Uuid);

impl TryFrom<&str> for Uuid {
	type Error = uuid::Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		uuid::Uuid::try_from(value).map(Uuid)
	}
}

impl Serialize for Uuid {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut buf = uuid::Uuid::encode_buffer();
		let s = self.0.hyphenated().encode_lower(&mut buf);
		serializer.serialize_str(s)
	}
}

impl<'de> Deserialize<'de> for Uuid {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Self::try_from(s.as_str()).map_err(|e| D::Error::custom(format!("uuid parsed failed: {e}")))
	}
}

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SaltPassword<'a>(Cow<'a, str>);

impl<'a> SaltPassword<'a> {
	pub fn new(password: &'a str, salt: Option<&'a str>) -> Self {
		match salt {
			Some(salt) => Self(Cow::Owned(format!("{salt}-{password}-{salt}"))),
			None => Self(Cow::Borrowed(password)),
		}
	}
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum Gender {
	Male,
	Female,
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "cup_op")]
#[sqlx(rename_all = "kebab-case")]
#[serde(rename_all = "camelCase")]
pub enum CupOp {
	PourIn,
	PourOut,
	Drink,
}
