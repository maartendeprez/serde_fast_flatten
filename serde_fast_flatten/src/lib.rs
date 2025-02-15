mod deserialize;
mod identifier;
mod serialize;

#[cfg(test)]
mod test;
pub mod test_structs;

pub use deserialize::{DeserializeFields, FieldDeserializer};
pub use identifier::{BorrowedFieldId, FieldId};
pub use serialize::SerializeFields;

#[cfg(feature = "derive")]
pub use serde_fast_flatten_derive::{DeserializeFields, SerializeFields};
