use fsffs::{Msg, Arthur, Proof, Elem};


#[derive(Msg)]
struct Pf {
    f: Elem<[u8; 32]>,
    v: Elem<u32>
}


#[derive(Msg)]
enum Pf3 {
    Var(Elem<u32>),
    X(Elem<u8>)
}

#[derive(Msg)]
struct Pf2([Elem<u8>; 32]);

impl Proof for Pf {
    type Proof = Self;
    type Statement = Elem<()>;
    type Error = ();

    fn verify<A: Arthur>(ts: &mut A, st: Self::Statement, pf: Self::Proof) -> Result<(), ()> {
        let v = ts.recv(pf.v);
        if v != 0 {
            let f = ts.recv(pf.f);
            println!("only read fields conditionally, still sound.");
        }
    
        Ok(())
    }
}


fn main() {
    let pf = Pf {
        f: [0; 32].into(),
        v: 8.into()
    };
    // pf.verify()
    println!("Hello, world!");
}
