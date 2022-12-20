use fsffs::{Arthur, Absorb, Msg, Proof, Tx, Sponge, Challenge};

use serde::{Serialize, Deserialize};

#[derive(Tx, Serialize, Deserialize)]
struct FieldElem {
    k: Msg<[u32; 4]>,
}

#[derive(Challenge)]
struct CC {
    v: u32,
    c: u8
}

#[derive(Tx)]
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

impl Proof for Pf {
    type Proof = Self;
    type Statement = ();
    type Error = ();

    fn interact<S: Sponge>(ts: &mut Arthur<S>, st: &Self::Statement, pf: Self::Proof) -> Result<(), ()> {
        let v = ts.recv(pf.v);
        if v != 0 {
            let f = ts.recv(pf.f);
            println!("only read fields conditionally, still sound.");
        }

        Ok(())
    }
}

fn main() {
    // pf.verify()
    println!("Hello, world!");
}
