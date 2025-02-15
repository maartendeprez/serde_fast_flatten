use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct A {
    num1: u64,
    #[serde(flatten)]
    b: B,
    #[serde(flatten)]
    d: D,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct B {
    num2: u64,
    #[serde(flatten)]
    c: C,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct C {
    num3: u64,
    num4: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct D {
    num5: u64,
    num6: u64,
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
