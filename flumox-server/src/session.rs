use std::str::FromStr;

use base64::{
    engine::{general_purpose::URL_SAFE_NO_PAD, GeneralPurpose},
    DecodeSliceError, Engine,
};
use getrandom::getrandom;
use serde::{Serialize, Serializer};
use thiserror::Error;

pub const SESSION_BYTES: usize = 16;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SessionToken(pub [u8; SESSION_BYTES]);

impl SessionToken {
    const BASE64: GeneralPurpose = URL_SAFE_NO_PAD;

    pub fn new() -> SessionToken {
        let mut buf = [0; SESSION_BYTES];
        getrandom(&mut buf).expect("failed to generate session token");
        SessionToken(buf)
    }
}

impl ToString for SessionToken {
    fn to_string(&self) -> String {
        SessionToken::BASE64.encode(self.0)
    }
}

impl FromStr for SessionToken {
    type Err = ParseSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; SESSION_BYTES];

        match SessionToken::BASE64.decode_slice(s, &mut buf) {
            Ok(SESSION_BYTES) => Ok(SessionToken(buf)),
            Ok(_) | Err(DecodeSliceError::OutputSliceTooSmall) => Err(ParseSessionError::BadLength),
            Err(DecodeSliceError::DecodeError(_)) => Err(ParseSessionError::InvalidBase64),
        }
    }
}

impl Serialize for SessionToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Copy, Error)]
pub enum ParseSessionError {
    #[error("invalid base 64")]
    InvalidBase64,
    #[error("bad length")]
    BadLength,
}
