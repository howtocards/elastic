use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use serde::de::{ Visitor, Error };
use super::mapping::{ TextMapping, DefaultTextMapping, TextFormat };
use ::mapping::ElasticType;

/// An Elasticsearch `text` field with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a `text` field with a mapping:
///
/// ```
/// use elastic_types::string::mapping::DefaultTextMapping;
/// use elastic_types::string::Text;
///
/// let string = Text::<DefaultTextMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Text<M> where
M: TextMapping {
	value: String,
	_m: PhantomData<M>
}
impl <M> Text<M> where
M: TextMapping {
	/// Creates a new `Text` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `Text` from a `String`:
	///
	/// ```
	/// use elastic_types::string::mapping::DefaultTextMapping;
	/// use elastic_types::string::Text;
	///
	/// let string = Text::<DefaultTextMapping>::new("my string");
	/// ```
	pub fn new<I>(string: I) -> Text<M> where I: Into<String> {
		Text {
			value: string.into(),
			_m: PhantomData
		}
	}

	/// Get the value of the string.
	pub fn get(&self) -> &str {
		&self.value
	}

	/// Set the value of the string.
	pub fn set<I>(&mut self, string: I) where I: Into<String> {
		self.value = string.into()
	}

	/// Change the mapping of this string.
	pub fn remap<MInto>(self) -> Text<MInto> where
	MInto: TextMapping {
		Text::<MInto>::new(self.value)
	}
}

impl_string_type!(Text, TextMapping, TextFormat, DefaultTextMapping);