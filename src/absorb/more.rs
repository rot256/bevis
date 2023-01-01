use super::{Absorb, Hasher};

impl Absorb<u8> for [u8] {
    fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
        self.len().absorb(h);
        h.write(self)
    }
}

impl Absorb<u8> for str {
    fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
        self.len().absorb(h);
        h.write(self.as_bytes())
    }
}
