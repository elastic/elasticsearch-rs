use crate::ElasticsearchError;
use bytes::buf::ext::Writer;
use bytes::buf::BufMutExt;
use bytes::{BufMut, Bytes, BytesMut};
use serde::{de::DeserializeOwned, Serialize};

/// The body of an API call that can be written to a buffer
pub trait Body {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError>;
}

/// A JSON body for an API call
pub struct JsonBody<T>(T);

impl<T> JsonBody<T>
where
    T: Serialize,
{
    pub fn new(t: T) -> Self {
        Self(t)
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

impl<T> Body for JsonBody<T>
where
    T: Serialize,
{
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        let writer = bytes.writer();
        match serde_json::to_writer(writer, &self.0) {
            Ok(_) => Ok(()),
            Err(e) => Err(ElasticsearchError::JsonError(e)),
        }
    }
}

/// A newline delimited body for an API call
pub struct NdBody<T>(pub(crate) Vec<T>);

impl<T> NdBody<T>
where
    T: Body,
{
    pub fn new(b: Vec<T>) -> Self {
        Self(b)
    }
}

// Accepts T as Body as opposed to Serialize, because each T may need to
// serialize to newline delimited in future e.g. A BulkOperation struct
// that implements Body could contain the operation and a document.
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
        bytes.resize(self.len(), 0);
        bytes.copy_from_slice(&self[..]);
        Ok(())
    }
}

impl Body for Vec<u8> {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        bytes.resize(self.len(), 0);
        bytes.copy_from_slice(&self[..]);
        Ok(())
    }
}

impl Body for &'static [u8] {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        bytes.resize(self.len(), 0);
        bytes.copy_from_slice(&self[..]);
        Ok(())
    }
}

impl Body for String {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        bytes.resize(self.len(), 0);
        bytes.copy_from_slice(self.as_bytes());
        Ok(())
    }
}

impl Body for &'static str {
    fn write(&self, bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        bytes.resize(self.len(), 0);
        bytes.copy_from_slice(self.as_bytes());
        Ok(())
    }
}

impl Body for () {
    fn write(&self, _bytes: &mut BytesMut) -> Result<(), ElasticsearchError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Body, Bytes, JsonBody, NdBody};
    use bytes::BytesMut;

    #[test]
    fn serialize_into_jsonbody_writes_to_bytes() -> Result<(), failure::Error> {
        let mut bytes = BytesMut::with_capacity(21);
        let body: JsonBody<_> = json!({"foo":"bar","baz":1}).into();
        let _ = body.write(&mut bytes)?;
        // NOTE: serde_json writes properties lexicographically
        assert_eq!(b"{\"baz\":1,\"foo\":\"bar\"}", &bytes[..]);

        Ok(())
    }

    #[test]
    fn bodies_into_ndbody_writes_to_bytes() -> Result<(), failure::Error> {
        let mut bytes = BytesMut::with_capacity(22);
        let mut bodies: Vec<JsonBody<_>> = Vec::with_capacity(2);
        bodies.push(json!({"item":1}).into());
        bodies.push(json!({"item":2}).into());

        let body = NdBody(bodies);
        let _ = body.write(&mut bytes)?;
        assert_eq!(b"{\"item\":1}\n{\"item\":2}\n", &bytes[..]);

        Ok(())
    }

    #[test]
    fn bytes_body_writes_to_bytes_mut() -> Result<(), failure::Error> {
        let mut bytes_mut = BytesMut::with_capacity(21);
        let bytes = bytes::Bytes::from(&b"{\"foo\":\"bar\",\"baz\":1}"[..]);
        let _ = bytes.write(&mut bytes_mut)?;
        assert_eq!(&bytes[..], &bytes_mut[..]);

        Ok(())
    }

    #[test]
    fn vec_body_writes_to_bytes_mut() -> Result<(), failure::Error> {
        let mut bytes_mut = BytesMut::with_capacity(21);
        let bytes = b"{\"foo\":\"bar\",\"baz\":1}".to_vec();
        let _ = bytes.write(&mut bytes_mut)?;
        assert_eq!(&bytes[..], &bytes_mut[..]);

        Ok(())
    }

    #[test]
    fn bytes_slice_body_writes_to_bytes_mut() -> Result<(), failure::Error> {
        let mut bytes_mut = BytesMut::with_capacity(21);
        let bytes: &'static [u8] = b"{\"foo\":\"bar\",\"baz\":1}";
        let _ = bytes.write(&mut bytes_mut)?;
        assert_eq!(&bytes[..], &bytes_mut[..]);

        Ok(())
    }

    #[test]
    fn string_body_writes_to_bytes_mut() -> Result<(), failure::Error> {
        let mut bytes_mut = BytesMut::with_capacity(21);
        let s = String::from("{\"foo\":\"bar\",\"baz\":1}");
        let _ = s.write(&mut bytes_mut)?;
        assert_eq!(s.as_bytes(), &bytes_mut[..]);

        Ok(())
    }

    #[test]
    fn string_slice_body_writes_to_bytes_mut() -> Result<(), failure::Error> {
        let mut bytes_mut = BytesMut::with_capacity(21);
        let s: &'static str = "{\"foo\":\"bar\",\"baz\":1}";
        let _ = s.write(&mut bytes_mut)?;
        assert_eq!(s.as_bytes(), &bytes_mut[..]);

        Ok(())
    }
}
