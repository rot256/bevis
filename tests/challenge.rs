#![allow(clippy::all)]
#![allow(dead_code)]

use bevis::{Absorb, Arthur, Challenge, Msg, Proof, Sponge, Tx};

use serde::{Deserialize, Serialize};

#[derive(Tx, Serialize, Deserialize)]
struct FieldElem {
    k: Msg<[u32; 4]>,
    v: Msg<Vec<u8>>,
}

#[derive(Challenge)]
struct CC {
    v: u32,
    c: u8,
}

#[derive(Challenge)]
struct CCC {
    v: [u8; 128],
    c: u8,
}

#[derive(Challenge)]
struct CCCC;

#[derive(Tx, Serialize, Deserialize)]
struct Round1 {
    v: Msg<u32>,
    r: Msg<FieldElem>,
}

#[derive(Absorb)]
enum Test {
    A(u32, Option<bool>),
    B(u64, Msg<u32>),
}

#[derive(Absorb)]
struct A(u32, u64);

#[derive(Tx)]
struct W(Msg<A>);

#[derive(Tx)]
struct Pf {
    f: Msg<Round1>,
    v: Msg<u32>,
    w: Msg<u8>,
}

#[derive(Absorb)]
enum V {
    A,
    B,
    C,
}

#[test]
fn test() {}
