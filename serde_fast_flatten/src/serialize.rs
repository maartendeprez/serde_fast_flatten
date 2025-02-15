use serde::ser::SerializeStruct;

pub trait SerializeFields {
    const NUM_FIELDS: usize;
    //fn num_fields(&self) -> usize;
    fn serialize_fields<S: SerializeStruct>(&self, s: &mut S) -> Result<(), S::Error>;
}
