use base64::write::EncoderWriter as Base64Encoder;
use std::io::Write;

pub fn base_64_encode_credentials(user: &str, password: &str) -> String {
    let mut value = Vec::new();
    {
        let mut encoder = Base64Encoder::new(&mut value, base64::STANDARD);
        write!(encoder, "{}:", user).unwrap();
        write!(encoder, "{}", password).unwrap();
    };
    String::from_utf8(value).unwrap()
}
