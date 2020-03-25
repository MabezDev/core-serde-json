use serde::ser;


use crate::ser::{Error, Result, Serializer};

pub struct SerializeSeq<'a, 'b>
{
    de: &'a mut Serializer<'b>,
    first: bool,
}

impl<'a, 'b> SerializeSeq<'a, 'b>
{
    pub(crate) fn new(de: &'a mut Serializer<'b>) -> Self {
        SerializeSeq { de, first: true }
    }
}

impl<'a, 'b> ser::SerializeSeq for SerializeSeq<'a, 'b>
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        if !self.first {
            self.de.buf.push(b',')?;
        }
        self.first = false;

        value.serialize(&mut *self.de)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.de.buf.push(b']')?;
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTuple for SerializeSeq<'a, 'b>
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}
