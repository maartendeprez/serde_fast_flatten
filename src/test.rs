use insta::{assert_binary_snapshot, assert_snapshot};

#[test]
fn compat_json() {
    let fast_flatten = crate::test_structs::fast_flatten::test_value();
    let serde = crate::test_structs::serde_flatten::test_value();
    let fast_flatten = serde_json::to_string(&fast_flatten).unwrap();
    let serde = serde_json::to_string(&serde).unwrap();
    assert_eq!(fast_flatten, serde)
}

#[test]
fn compat_yaml() {
    let fast_flatten = crate::test_structs::fast_flatten::test_value();
    let serde = crate::test_structs::serde_flatten::test_value();
    let fast_flatten = serde_yaml::to_string(&fast_flatten).unwrap();
    let serde = serde_yaml::to_string(&serde).unwrap();
    assert_eq!(fast_flatten, serde)
}

// Output differs.
// #[test]
// fn compat_cbor() {
//     let fast_flatten = crate::test_structs::fast_flatten::test_value();
//     let serde = crate::test_structs::serde_flatten::test_value();
//     let fast_flatten = serde_cbor::to_vec(&fast_flatten).unwrap();
//     let serde = serde_cbor::to_vec(&serde).unwrap();
//     assert_eq!(fast_flatten, serde)
// }

// Output differs.
// #[test]
// fn compat_cbor_packed() {
//     let fast_flatten = crate::test_structs::fast_flatten::test_value();
//     let serde = crate::test_structs::serde_flatten::test_value();
//     let fast_flatten = serde_cbor::ser::to_vec_packed(&fast_flatten).unwrap();
//     let serde = serde_cbor::ser::to_vec_packed(&serde).unwrap();
//     assert_eq!(fast_flatten, serde)
// }

// Bincode fails on standard auto-derived Serialize impl.
// #[test]
// fn compat_bincode() {
//     let fast_flatten = crate::test_structs::fast_flatten::test_value();
//     let serde = crate::test_structs::serde_flatten::test_value();
//     let fast_flatten = bincode::serialize(&fast_flatten).unwrap();
//     let serde = bincode::serialize(&serde).unwrap();
//     assert_eq!(fast_flatten, serde)
// }

// Bitcode fails on standard auto-derived Serialize impl.
// #[test]
// fn compat_bitcode() {
//     let fast_flatten = crate::test_structs::fast_flatten::test_value();
//     let serde = crate::test_structs::serde_flatten::test_value();
//     let fast_flatten = bitcode::serialize(&fast_flatten).unwrap();
//     let serde = bitcode::serialize(&serde).unwrap();
//     assert_eq!(fast_flatten, serde)
// }

#[test]
fn roundtrip_json() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = serde_json::to_string(&x).unwrap();
    let y = serde_json::from_str(&s).unwrap();
    assert_eq!(x, y);
    assert_snapshot!(s);
}

#[test]
fn roundtrip_yaml() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = serde_yaml::to_string(&x).unwrap();
    let y = serde_yaml::from_str(&s).unwrap();
    assert_eq!(x, y);
    assert_snapshot!(s);
}

#[test]
fn roundtrip_cbor() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = serde_cbor::to_vec(&x).unwrap();
    let y = serde_cbor::from_slice(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".cbor", s);
}

#[test]
fn roundtrip_cbor_packed() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = serde_cbor::ser::to_vec_packed(&x).unwrap();
    let y = serde_cbor::from_slice(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".cbor", s);
}

#[test]
fn roundtrip_bincode() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = bincode::serialize(&x).unwrap();
    let y = bincode::deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bincode", s);
}

#[test]
fn roundtrip_bincode_compact() {
    use bincode::Options;
    let opts = bincode::DefaultOptions::new();
    let x = crate::test_structs::fast_flatten::test_value();
    let s = opts.serialize(&x).unwrap();
    let y = opts.deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bincode", s);
}

#[test]
fn roundtrip_bitcode() {
    let x = crate::test_structs::fast_flatten::test_value();
    let s = bitcode::serialize(&x).unwrap();
    let y = bitcode::deserialize(&s).unwrap();
    assert_eq!(x, y);
    assert_binary_snapshot!(".bitcode", s);
}
