mod serialize;

// more absorb impl. (beyond serde serializable types)
mod more;

use serde::Serialize;

use serialize::AbsorbSerializer;


pub trait Hasher<W> {
    fn write(&mut self, buf: &[W]);
}

pub trait Absorb<W> {
    fn absorb<H: Hasher<W>>(&self, h: &mut H);
}

impl<T: Serialize> Absorb<u8> for T {
    fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
        self.serialize(&mut AbsorbSerializer { h }).unwrap();
    }
}
