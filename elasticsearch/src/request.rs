use crate::ElasticsearchError;
use bytes::{BufMut, Bytes, BytesMut, IntoBuf};
use serde::{de::DeserializeOwned, Serialize};

/// The body of an API call that can be written to a buffer
pub trait Body {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError>;
}

pub struct JsonBody<T>(T);

/// The body of an API call that implements Serialize
impl<T> Body for JsonBody<T>
where
    T: Serialize,
{
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        let mut writer = bytes.writer();
        match serde_json::to_writer(&mut writer, &self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ElasticsearchError::JsonError(e)),
        }
    }
}

impl<T> From<T> for JsonBody<T>
where
    T: Serialize,
{
    fn from(t: T) -> Self {
        JsonBody(t)
    }
}

pub struct NdBody<T>(pub Vec<T>);

/// A newline delimited body of an API call.
/// Accepts T as Body as opposed to Serialize, because each T may need to
/// serialize to newline delimited.
impl<T> Body for NdBody<T>
where
    T: Body,
{
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        for line in &self.0 {
            line.write(bytes)?;
            bytes.put_u8(b'\n');
        }
        Ok(())
    }
}

impl Body for Bytes {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        bytes.copy_from_slice(&self.to_vec());
        Ok(())
    }
}

impl Body for () {
    fn write(&self, _bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        Ok(())
    }
}
