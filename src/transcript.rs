use crate::{Absorb, Hasher, Tx};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use serde::Serialize;


use core::fmt;
use std::error::Error;

use serde::ser;

/*
#[macro_export]
macro_rules! absorb_int_impl {
    ( $t:tt ) => {
        impl Absorb for $t {
            #[inline(always)]
            fn absorb<H: Hasher>(&self, h: &mut H) {
                h.write(&self.to_le_bytes())
            }
        }
    };
}


absorb_int_impl!(i8);
absorb_int_impl!(i16);
absorb_int_impl!(i32);
absorb_int_impl!(i64);
absorb_int_impl!(i128);

absorb_int_impl!(u8);
absorb_int_impl!(u16);
absorb_int_impl!(u32);
absorb_int_impl!(u64);
absorb_int_impl!(u128);
*/

struct AbsorbSerializer<'a, H: Hasher> {
    h: &'a mut H
}

struct AbsorbCompound<'a, 'b, H: Hasher> {
    ser: &'b mut AbsorbSerializer<'a, H>,
}

#[derive(Debug)]
struct AbsorbError {}

impl fmt::Display for AbsorbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AbsorbError")
    }
}

impl Error for AbsorbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ser::Error for AbsorbError {
    fn custom<T>(_msg: T) -> Self {
        Self{ }
    }
}

impl <'a, H: Hasher> AbsorbSerializer<'a, H> {
    fn serialize_byte(&mut self, v: u8) {
        self.h.write(&[v]);
    }
}

#[macro_export]
macro_rules! absorb_int_impl {
    ( $name:ident, $t:tt ) => {
        fn $name(self, v: $t) -> Result<(), Self::Error> {
            self.h.write(&v.to_le_bytes());
            Ok(())
        }
    };
}

fn write_len<H: Hasher>(h: &mut H, l: usize) {
    h.write(&l.to_le_bytes())
}

impl<'a, 'b, H: Hasher> serde::Serializer for &'b mut AbsorbSerializer<'a, H> {
    type Ok = ();
    type Error = AbsorbError;

    type SerializeSeq = AbsorbCompound<'a, 'b, H>;
    type SerializeTuple = AbsorbCompound<'a, 'b, H>;
    type SerializeTupleStruct = AbsorbCompound<'a, 'b, H>;
    type SerializeTupleVariant = AbsorbCompound<'a, 'b, H>;
    type SerializeMap = AbsorbCompound<'a, 'b, H>;
    type SerializeStruct = AbsorbCompound<'a, 'b, H>;
    type SerializeStructVariant = AbsorbCompound<'a, 'b, H>;

    fn serialize_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.serialize_byte(v as u8);
        Ok(())
    }

    absorb_int_impl!(serialize_i8, i8);
    absorb_int_impl!(serialize_i16, i16);
    absorb_int_impl!(serialize_i32, i32);
    absorb_int_impl!(serialize_i64, i64);

    absorb_int_impl!(serialize_u8, u8);
    absorb_int_impl!(serialize_u16, u16);
    absorb_int_impl!(serialize_u32, u32);
    absorb_int_impl!(serialize_u64, u64);

    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, c: char) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.h.write(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        write_len(self.h, v.len());
        self.h.write(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_byte(0);
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_byte(1);
        v.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.expect("sequence must have length");
        write_len(self.h, len);
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index);
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.expect("sequence must have length");
        self.serialize_u64(len as u64);
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index);
        Ok(AbsorbCompound{ ser: self })
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        self.serialize_u32(variant_index);
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.serialize_u32(variant_index)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}


impl<'a, 'b, H: Hasher> ser::SerializeSeq for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeTuple for AbsorbCompound<'a, 'b, H>  {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeTupleStruct for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeTupleVariant for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeMap for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeStruct for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, 'b, H: Hasher> ser::SerializeStructVariant for AbsorbCompound<'a, 'b, H> {
    type Ok = ();
    type Error = AbsorbError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T: Serialize> Absorb for T {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        let mut h = AbsorbSerializer { h };
        let _r = self.serialize(&mut h);
    }
}

/*
impl<T: Tx> Absorb for T {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.read(h)
    }
}

impl Absorb for () {
    fn absorb<H: Hasher>(&self, _h: &mut H) {}
}

impl Absorb for bool {
    #[inline(always)]
    fn absorb<H: Hasher>(&self, h: &mut H) {
        let bit: u8 = if *self { 1 } else { 0 };
        h.write(&[bit]);
    }
}

impl<T: Absorb> Absorb for Vec<T> {
    #[inline(always)]
    fn absorb<H: Hasher>(&self, h: &mut H) {
        let s: &[T] = &self[..];
        s.absorb(h)
    }
}

impl<T: Absorb> Absorb for [T] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read the length
        // TOOD: wait for https://github.com/rust-lang/rust/issues/96762
        let n = (self.len() as u64).to_le_bytes();
        n.absorb(h);

        // read every element
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}

impl<T: Absorb> Absorb for Option<T> {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read if Some/None
        self.is_some().absorb(h);

        // read inner value (if present)
        if let Some(v) = self {
            v.absorb(h)
        }
    }
}

impl<A: Absorb, B: Absorb> Absorb for Result<A, B> {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read if Ok/Err
        self.is_ok().absorb(h);

        // read inner value (if present)
        match self {
            Ok(v) => v.absorb(h),
            Err(e) => e.absorb(h),
        }
    }
}

impl<const N: usize, T: Absorb> Absorb for [T; N] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read every element 
        // (the length is fixed so no need to include it)
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}
*/