use serde::de::MapAccess;

use crate::FieldId;

pub trait DeserializeFields<'de>: serde::de::Deserialize<'de> {
    type FieldDeserializer: FieldDeserializer<'de, Value = Self>;
}

pub trait FieldDeserializer<'de>: Default {
    type Value;

    // fn wants_field(&self, field: BorrowedFieldId<'_>) -> bool;

    fn deserialize_field<A: MapAccess<'de>>(
        &mut self,
        field: FieldId<'de>,
        map: &mut A,
    ) -> Result<Result<(), FieldId<'de>>, A::Error>;

    fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E>;
}
