use std::{
    borrow::{Borrow, Cow},
    marker::PhantomData,
};

use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub enum FieldId<'a> {
    String(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
    U64(u64),
}

#[derive(Clone, Copy, Debug)]
pub enum BorrowedFieldId<'a> {
    String(&'a str),
    Bytes(&'a [u8]),
    U64(u64),
}

impl FieldId<'_> {
    pub fn offset(self, n: u64) -> Self {
        match self {
            Self::U64(id) => Self::U64(id.saturating_sub(n)),
            x => x,
        }
    }

    pub fn borrow(&self) -> BorrowedFieldId {
        match self {
            FieldId::String(cow) => BorrowedFieldId::String(cow.borrow()),
            FieldId::Bytes(cow) => BorrowedFieldId::Bytes(cow.borrow()),
            FieldId::U64(id) => BorrowedFieldId::U64(*id),
        }
    }
}

impl<'de> Deserialize<'de> for FieldId<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FieldIdVisitor<'a>(PhantomData<&'a ()>);

        impl<'de> Visitor<'de> for FieldIdVisitor<'de> {
            type Value = FieldId<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "field identifier")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::String(Cow::Borrowed(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::String(Cow::Owned(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::String(Cow::Owned(v.to_string())))
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::Bytes(Cow::Borrowed(v)))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::Bytes(Cow::Owned(v)))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::Bytes(Cow::Owned(v.to_vec())))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldId::U64(v))
            }
        }

        deserializer.deserialize_identifier(FieldIdVisitor(PhantomData))
    }
}
