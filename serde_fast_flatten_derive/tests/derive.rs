use insta::{assert_binary_snapshot, assert_snapshot};
use serde_fast_flatten_derive::{DeserializeFields, SerializeFields};

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct Address {
    #[serde(flatten)]
    person: Person,
    #[serde(flatten)]
    street_address: StreetAddress,
}

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct StreetAddress {
    street: String,
    house_number: u64,
    #[serde(flatten)]
    city_address: CityAddress,
}

#[derive(SerializeFields, DeserializeFields, PartialEq, Eq, Debug)]
struct CityAddress {
    city: String,
    postal_code: String,
    state: Option<String>,
    country: String,
}

fn test_value() -> Address {
    Address {
        person: Person {
            first_name: String::from("John"),
            last_name: String::from("Lennon"),
        },
        street_address: StreetAddress {
            street: String::from("Long and Winding Road"),
            house_number: 42,
            city_address: CityAddress {
                city: String::from("Strawberry Fields"),
                postal_code: String::from("1234"),
                state: None,
                country: String::from("Nowhere Land"),
            },
        },
    }
}

#[test]
fn roundtrip_json() {
    let x = test_value();
    let s = serde_json::to_string(&x).unwrap();
    let y = serde_json::from_str(&s).unwrap();
    assert_eq!(x, y);
    assert_snapshot!(s);
}

#[test]
fn roundtrip_yaml() {
    let x = test_value();
    let s = serde_yaml::to_string(&x).unwrap();
    let y = serde_yaml::from_str(&s).unwrap();
    assert_eq!(x, y);
    assert_snapshot!(s);
}

#[test]
fn roundtrip_cbor() {
    let x = test_value();
    let s = serde_cbor::to_vec(&x).unwrap();
    let y = serde_cbor::from_slice(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".cbor", s);
}

#[test]
fn roundtrip_cbor_packed() {
    let x = test_value();
    let s = serde_cbor::ser::to_vec_packed(&x).unwrap();
    let y = serde_cbor::from_slice(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".cbor", s);
}

#[test]
fn roundtrip_bincode() {
    let x = test_value();
    let s = bincode::serialize(&x).unwrap();
    let y = bincode::deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bincode", s);
}

#[test]
fn roundtrip_bincode_compact() {
    use bincode::Options;
    let opts = bincode::DefaultOptions::new();
    let x = test_value();
    let s = opts.serialize(&x).unwrap();
    let y = opts.deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bincode", s);
}

#[test]
fn roundtrip_bitcode() {
    let x = test_value();
    let s = bitcode::serialize(&x).unwrap();
    let y = bitcode::deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bitcode", s);
}
