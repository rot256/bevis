use crate::{Absorb, Hasher};

use serde::Serialize;

use core::fmt;

use serde::ser;

pub(super) struct AbsorbSerializer<'a, H: Hasher<u8>> {
    pub h: &'a mut H,
}

pub(super) struct AbsorbCompound<'a, 'b, H: Hasher<u8>> {
    ser: &'b mut AbsorbSerializer<'a, H>,
}

#[derive(Debug)]
pub(super) struct AbsorbError {}

impl fmt::Display for AbsorbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AbsorbError")
    }
}

impl ser::StdError for AbsorbError {}

impl ser::Error for AbsorbError {
    fn custom<T>(_msg: T) -> Self {
        Self {}
    }
}

impl<'a, H: Hasher<u8>> AbsorbSerializer<'a, H> {
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

impl<'a, 'b, H: Hasher<u8>> serde::Serializer for &'b mut AbsorbSerializer<'a, H> {
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

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: fmt::Display,
    {
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<(), Self::Error> {
        self.h.write(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<(), Self::Error> {
        self.h.write(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<(), Self::Error> {
        let mut buf: [u8; 4] = [0u8; 4];
        value.encode_utf8(&mut buf);
        self.h.write(&buf);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        v.len().absorb(self.h);
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
        len.absorb(self.h);
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.expect("sequence must have length");
        len.absorb(self.h);
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(AbsorbCompound { ser: self })
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
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
        self.serialize_u32(variant_index)?;
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeSeq for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeTuple for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeTupleStruct for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeTupleVariant for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeMap for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeStruct for AbsorbCompound<'a, 'b, H> {
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

impl<'a, 'b, H: Hasher<u8>> ser::SerializeStructVariant for AbsorbCompound<'a, 'b, H> {
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