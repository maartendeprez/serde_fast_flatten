use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};

use crate::{BorrowedFieldId, DeserializeFields, FieldDeserializer, FieldId, SerializeFields};

#[derive(PartialEq, Eq, Debug)]
pub struct A {
    num1: u64,
    //#[serde(flatten)]
    b: B,
    //#[serde(flatten)]
    d: D,
}

#[derive(PartialEq, Eq, Debug)]
struct B {
    num2: u64,
    //#[serde(flatten)]
    c: C,
}

#[derive(PartialEq, Eq, Debug)]
struct C {
    num3: u64,
    num4: u64,
}

#[derive(PartialEq, Eq, Debug)]
struct D {
    num5: u64,
    num6: u64,
}

impl Serialize for A {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("A", Self::NUM_FIELDS)?;
        self.serialize_fields(&mut s)?;
        s.end()
    }
}

impl SerializeFields for A {
    const NUM_FIELDS: usize = 1 + B::NUM_FIELDS + D::NUM_FIELDS;

    #[inline(always)]
    fn serialize_fields<S: SerializeStruct>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_field("num1", &self.num1)?;
        self.b.serialize_fields(s)?;
        self.d.serialize_fields(s)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct AFields<'de> {
    num1: Option<u64>,
    b: <B as DeserializeFields<'de>>::FieldDeserializer,
    d: <D as DeserializeFields<'de>>::FieldDeserializer,
}

impl<'de> Deserialize<'de> for A {
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AVisitor;

        impl<'de> Visitor<'de> for AVisitor {
            type Value = A;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct A")
            }

            #[inline(always)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut fields = AFields::default();
                while let Some(field) = map.next_key::<FieldId<'de>>()? {
                    if let Err(_field) = fields.deserialize_field(field, &mut map)? {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                fields.finish()
            }

            #[inline(always)]
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let num1 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num1`")
                })?;
                let b = seq
                    .next_element()?
                    .ok_or_else(|| <S::Error as serde::de::Error>::custom("missing field `b`"))?;
                let d = seq
                    .next_element()?
                    .ok_or_else(|| <S::Error as serde::de::Error>::custom("missing field `d`"))?;
                Ok(A { num1, b, d })
            }
        }

        const FIELDS: &[&str] = &["num1", "b", "d"];
        deserializer.deserialize_struct("A", FIELDS, AVisitor)
    }
}

impl<'de> DeserializeFields<'de> for A {
    type FieldDeserializer = AFields<'de>;
}

impl<'de> FieldDeserializer<'de> for AFields<'de> {
    type Value = A;

    #[inline(always)]
    fn deserialize_field<A: MapAccess<'de>>(
        &mut self,
        field: FieldId<'de>,
        map: &mut A,
    ) -> Result<Result<(), FieldId<'de>>, A::Error> {
        match field.borrow() {
            BorrowedFieldId::String("num1")
            | BorrowedFieldId::Bytes(b"num1")
            | BorrowedFieldId::U64(0) => {
                self.num1 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            _ => match self.b.deserialize_field(field.offset(1), map)? {
                Ok(()) => Ok(Ok(())),
                Err(field) => self.d.deserialize_field(field, map),
            },
        }
    }

    #[inline(always)]
    fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(A {
            num1: self.num1.ok_or_else(|| E::custom("missing field `num1`"))?,
            b: self.b.finish()?,
            d: self.d.finish()?,
        })
    }
}

impl Serialize for B {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("B", Self::NUM_FIELDS)?;
        self.serialize_fields(&mut s)?;
        s.end()
    }
}

impl SerializeFields for B {
    const NUM_FIELDS: usize = 1 + C::NUM_FIELDS;

    #[inline(always)]
    fn serialize_fields<S: SerializeStruct>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_field("num2", &self.num2)?;
        self.c.serialize_fields(s)?;
        Ok(())
    }
}

#[derive(Default)]
struct BFields<'de> {
    num2: Option<u64>,
    c: <C as DeserializeFields<'de>>::FieldDeserializer,
}

impl<'de> Deserialize<'de> for B {
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BVisitor;

        impl<'de> Visitor<'de> for BVisitor {
            type Value = B;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct B")
            }

            #[inline(always)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut fields = BFields::default();
                while let Some(field) = map.next_key::<FieldId<'de>>()? {
                    if let Err(_field) = fields.deserialize_field(field, &mut map)? {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                fields.finish()
            }

            #[inline(always)]
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let num2 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num2`")
                })?;
                let c = seq
                    .next_element()?
                    .ok_or_else(|| <S::Error as serde::de::Error>::custom("missing field `c`"))?;
                Ok(B { num2, c })
            }
        }

        const FIELDS: &[&str] = &["num2", "c"];
        deserializer.deserialize_struct("B", FIELDS, BVisitor)
    }
}

impl<'de> DeserializeFields<'de> for B {
    type FieldDeserializer = BFields<'de>;
}

impl<'de> FieldDeserializer<'de> for BFields<'de> {
    type Value = B;

    #[inline(always)]
    fn deserialize_field<A: MapAccess<'de>>(
        &mut self,
        field: FieldId<'de>,
        map: &mut A,
    ) -> Result<Result<(), FieldId<'de>>, A::Error> {
        match field.borrow() {
            BorrowedFieldId::String("num2")
            | BorrowedFieldId::Bytes(b"num2")
            | BorrowedFieldId::U64(0) => {
                self.num2 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            _ => self.c.deserialize_field(field.offset(1), map),
        }
    }

    #[inline(always)]
    fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(B {
            num2: self.num2.ok_or_else(|| E::custom("missing field `num2`"))?,
            c: self.c.finish()?,
        })
    }
}

impl Serialize for C {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("C", Self::NUM_FIELDS)?;
        self.serialize_fields(&mut s)?;
        s.end()
    }
}

impl SerializeFields for C {
    const NUM_FIELDS: usize = 2;

    #[inline(always)]
    fn serialize_fields<S: SerializeStruct>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_field("num3", &self.num3)?;
        s.serialize_field("num4", &self.num4)?;
        Ok(())
    }
}

#[derive(Default)]
struct CFields {
    num3: Option<u64>,
    num4: Option<u64>,
}

impl<'de> Deserialize<'de> for C {
    #[inline(always)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CVisitor;

        impl<'de> Visitor<'de> for CVisitor {
            type Value = C;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct C")
            }

            #[inline(always)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut fields = CFields::default();
                while let Some(field) = map.next_key::<FieldId<'de>>()? {
                    if let Err(_field) = fields.deserialize_field(field, &mut map)? {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                fields.finish()
            }

            #[inline(always)]
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let num3 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num3`")
                })?;
                let num4 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num4`")
                })?;
                Ok(C { num3, num4 })
            }
        }

        const FIELDS: &[&str] = &["num3", "num4"];
        deserializer.deserialize_struct("C", FIELDS, CVisitor)
    }
}

impl DeserializeFields<'_> for C {
    type FieldDeserializer = CFields;
}

impl<'de> FieldDeserializer<'de> for CFields {
    type Value = C;

    #[inline(always)]
    fn deserialize_field<A: MapAccess<'de>>(
        &mut self,
        field: FieldId<'de>,
        map: &mut A,
    ) -> Result<Result<(), FieldId<'de>>, A::Error> {
        match field.borrow() {
            BorrowedFieldId::String("num3")
            | BorrowedFieldId::Bytes(b"num3")
            | BorrowedFieldId::U64(0) => {
                self.num3 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            BorrowedFieldId::String("num4")
            | BorrowedFieldId::Bytes(b"num4")
            | BorrowedFieldId::U64(1) => {
                self.num4 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            _ => Ok(Err(field.offset(2))),
        }
    }

    #[inline(always)]
    fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(C {
            num3: self.num3.ok_or_else(|| E::custom("missing field `num3`"))?,
            num4: self.num4.ok_or_else(|| E::custom("missing field `num4`"))?,
        })
    }
}

impl Serialize for D {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("C", Self::NUM_FIELDS)?;
        self.serialize_fields(&mut s)?;
        s.end()
    }
}

impl SerializeFields for D {
    const NUM_FIELDS: usize = 2;

    #[inline(always)]
    fn serialize_fields<S: SerializeStruct>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_field("num5", &self.num5)?;
        s.serialize_field("num6", &self.num6)?;
        Ok(())
    }
}

#[derive(Default)]
struct DFields {
    num5: Option<u64>,
    num6: Option<u64>,
}

impl<'de> Deserialize<'de> for D {
    #[inline(always)]
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        struct DVisitor;

        impl<'de> Visitor<'de> for DVisitor {
            type Value = D;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct C")
            }

            #[inline(always)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut fields = DFields::default();
                while let Some(field) = map.next_key::<FieldId<'de>>()? {
                    if let Err(_field) = fields.deserialize_field(field, &mut map)? {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                fields.finish()
            }

            #[inline(always)]
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let num5 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num5`")
                })?;
                let num6 = seq.next_element()?.ok_or_else(|| {
                    <S::Error as serde::de::Error>::custom("missing field `num6`")
                })?;
                Ok(D { num5, num6 })
            }
        }

        const FIELDS: &[&str] = &["num5", "num6"];
        deserializer.deserialize_struct("D", FIELDS, DVisitor)
    }
}

impl DeserializeFields<'_> for D {
    type FieldDeserializer = DFields;
}

impl<'de> FieldDeserializer<'de> for DFields {
    type Value = D;

    #[inline(always)]
    fn deserialize_field<A: MapAccess<'de>>(
        &mut self,
        field: FieldId<'de>,
        map: &mut A,
    ) -> Result<Result<(), FieldId<'de>>, A::Error> {
        match field.borrow() {
            BorrowedFieldId::String("num5")
            | BorrowedFieldId::Bytes(b"num5")
            | BorrowedFieldId::U64(0) => {
                self.num5 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            BorrowedFieldId::String("num6")
            | BorrowedFieldId::Bytes(b"num6")
            | BorrowedFieldId::U64(1) => {
                self.num6 = Some(map.next_value()?);
                Ok(Ok(()))
            }
            _ => Ok(Err(field.offset(2))),
        }
    }

    #[inline(always)]
    fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(D {
            num5: self.num5.ok_or_else(|| E::custom("missing field `num5`"))?,
            num6: self.num6.ok_or_else(|| E::custom("missing field `num6`"))?,
        })
    }
}

pub fn test_value() -> A {
    A {
        num1: 1,
        b: B {
            num2: 2,
            c: C { num3: 3, num4: 4 },
        },
        d: D { num5: 5, num6: 6 },
    }
}
