use super::{Absorb, Hasher};

/*
// a more general impl.
impl <T: Absorb> Absorb for [T] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.len().absorb(h);
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}
*/

impl Absorb for [u8] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.len().absorb(h);
        h.write(self)
    }
}

impl Absorb for str {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.len().absorb(h);
        h.write(self.as_bytes())
    }
}
