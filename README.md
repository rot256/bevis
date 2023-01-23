# Bevis; Fiat-Shamir Without The Hassle

(bevis)[https://en.wiktionary.org/wiki/bevis] (be- +‎ vis) meaning "proof" in the Scandinavian languages,
aims to be the universal standard for how to implement public-coin arguments in the Rust ecosystem.
Bevis aims to have minimal dependencies (serde and rand_core) and no/very minimal run-time overhead,
consisting primarily of a collection of types, traits and procedural macros.

Here is a simplified example of implementing Schnorr:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Comm {
    v: RistrettoPoint,
}

#[derive(Serialize, Deserialize)]
struct Resp {
    z: Scalar
}

#[derive(Serialize, Deserialize)]
struct Pf {
    a: Msg<Comm>,
    z: Resp,
}

fn verify<T: Transcript>(ts: &mut T, pk: &RistrettoPoint, pf: Pf) -> bool {
    // add the public key
    ts.append(pk);

    // receive first round msg
    let a = ts.recv(pf.a);

    // get a challenge (transcript impl. RngCore and CryptoRng)
    let c = Scalar::random(ts);

    // check
    c * pk + pf.a == pf.z * 
}
```
