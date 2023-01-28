mod serialize;

// more absorb impl. (beyond serde serializable types)
mod more;

use serde::Serialize;

use serialize::AbsorbSerializer;

pub trait Hasher {
    fn write(&mut self, buf: &[u8]);
}

pub trait Absorb {
    fn absorb<H: Hasher>(&self, h: &mut H);
}

impl<T: Serialize> Absorb for T {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.serialize(&mut AbsorbSerializer { h }).unwrap();
    }
}
