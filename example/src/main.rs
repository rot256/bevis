use fsffs::{Msg, Arthur, Proof, Elem};


#[derive(Msg)]
struct Pf {
    f: Elem<[u8; 32]>,
    v: Elem<u32>
}

#[derive(Msg)]
struct Pf2([Elem<u8>; 32]);

impl Proof for Pf {
    fn verify<A: Arthur>(self, ts: &mut A) -> bool {
        let v = ts.recv(self.v);
        if v != 0 {
            let f = ts.recv(self.f);
            println!("only read fields conditionally, still sound.");
        }
    
        true
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
