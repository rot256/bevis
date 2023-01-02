#![allow(clippy::all)]
#![allow(dead_code)]

use bevis::{Absorb, Arthur, Challenge, Msg, Proof, Sponge, Tx};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct U32(u32);

impl Absorb<u32> for U32 {
    fn absorb<H: bevis::Hasher<u32>>(&self, h: &mut H) {
        h.write(&[self.0])
    }
}

impl Challenge<u32> for U32 {
    fn sample<S: bevis::Sampler<u32>>(ts: &mut S) -> Self {
        U32(ts.read())
    }
}

impl Challenge<u8> for U32 {
    fn sample<S: bevis::Sampler<u8>>(ts: &mut S) -> Self {
        let v: [u8; 4] = bevis::Challenge::sample(ts);
        Self(u32::from_le_bytes(v))
    }
}

#[derive(Challenge, Serialize, Absorb)]
#[challenge(u32, u8)]
#[absorb(u32)]
struct FieldElem {
    k: [U32; 4],
    v: U32
}



/*
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
#[as_type(u8)]
enum Test {
    A(u32, Option<bool>),
    B(u64, Msg<u32>),
}

#[derive(Absorb)]
#[sponge(u32)]
struct A(u32, u64);

#[derive(Tx)]
struct W(Msg<A>);

#[derive(Tx)]
struct Pf {
    f: Msg<Round1>,
    v: Msg<u32>,
    w: Msg<u8>,
}

#[derive(Absorb, Challenge)]
#[absorb(u8)]
#[challenge(u8)]
enum V {
    A,
    B,
    C,
}
*/

#[test]
fn test() {}
