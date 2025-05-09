use serde::{Deserialize, Serialize, de::Error};

#[derive(Debug, Eq, PartialEq, sqlx::Type, Default)]
#[sqlx(transparent)]
pub struct Uuid(uuid::Uuid);

impl Serialize for Uuid {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut buf = uuid::Uuid::encode_buffer();
		let s = self.0.simple().encode_lower(&mut buf);
		serializer.serialize_str(s)
	}
}

impl<'de> Deserialize<'de> for Uuid {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		uuid::Uuid::parse_str(&s)
			.map(Uuid)
			.map_err(|e| D::Error::custom(format!("uuid parsed failed: {e}")))
	}
}
