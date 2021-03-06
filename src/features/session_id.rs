use std::io::{Read, Write};

use sigma_ser::vlq_encode::{ReadSigmaVlqExt, WriteSigmaVlqExt};

use crate::encoding::vlq::{default_vlq_reader, default_vlq_writer, TryFromVlq, TryIntoVlq};
use crate::models::MagicBytes;

use super::{FeatureParseError, FeatureSerializeError};

#[derive(Debug, PartialEq, Eq)]
pub struct SessionId {
    pub magic: MagicBytes,
    pub session_id: i64,
}

impl TryFromVlq for SessionId {
    type Error = FeatureParseError;

    fn try_from_vlq(data: Vec<u8>) -> Result<Self, Self::Error> {
        let mut vlq_reader = default_vlq_reader(data);

        let magic = {
            let mut m = MagicBytes::default();
            vlq_reader.read_exact(&mut m.0)?;
            m
        };
        let session_id = vlq_reader.get_i64()?;

        Ok(SessionId { magic, session_id })
    }
}

impl TryIntoVlq for SessionId {
    type Error = FeatureSerializeError;

    fn try_into_vlq(&self) -> Result<Vec<u8>, Self::Error> {
        let mut vlq_writer = default_vlq_writer(Vec::new());
        let SessionId { magic: MagicBytes(magic), session_id } = self;

        vlq_writer.write(magic)?;
        vlq_writer.put_i64(*session_id)?;

        Ok(vlq_writer.into_inner())
    }
}
