use super::{Absorb, Hasher};

impl<T: Absorb> Absorb for [T] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.len().absorb(h);
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}
