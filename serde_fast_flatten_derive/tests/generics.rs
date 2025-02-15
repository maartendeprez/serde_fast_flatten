use insta::assert_snapshot;
use serde_fast_flatten::{DeserializeFields, SerializeFields};

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct Paged<T, U> {
    items: Vec<T>,
    #[serde(flatten)]
    params: PageParams<U>,
}

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct PageParams<T> {
    start: T,
    limit: T,
}

#[test]
fn generics() {
    let x = Paged::<String, u64> {
        items: Vec::from_iter([String::from("item 1")]),
        params: PageParams {
            start: 0,
            limit: 10,
        },
    };
    let s = serde_json::to_string_pretty(&x).unwrap();
    let y = serde_json::from_str::<Paged<String, u64>>(&s).unwrap();
    assert_eq!(x, y);
    assert_snapshot!(s);
}
