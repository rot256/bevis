#![allow(clippy::all)]
#![allow(dead_code)]

use bevis::{Absorb, Arthur, Challenge, Msg, Proof, Sponge, Tx, Transcript};

use serde::{Deserialize, Serialize};

use bevis_strobe as strobe;

#[derive(Deserialize, Serialize)]
struct U32(u32);


#[derive(Serialize, Deserialize, Challenge)]
struct FieldElem {
    k: [u32; 4],
}

#[derive(Serialize, Deserialize)]
struct Round1 {
    a: FieldElem,
    b: FieldElem
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
fn test() {
    let ts = strobe::Transcript::new("test");
    let mut ts = bevis::TraceTranscript::new(ts);


    ts.append(&12u32);

    let _v: FieldElem = ts.challenge();

  
    ts.append(&_v);



    println!("{}", ts);

    println!("{}", ts.transcript());
}
