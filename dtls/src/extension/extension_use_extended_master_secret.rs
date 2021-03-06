#[cfg(test)]
mod extension_use_extended_master_secret_test;

use super::*;

const EXTENSION_USE_EXTENDED_MASTER_SECRET_HEADER_SIZE: usize = 4;

// https://tools.ietf.org/html/rfc8422
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionUseExtendedMasterSecret {
    pub(crate) supported: bool,
}

impl ExtensionUseExtendedMasterSecret {
    pub fn extension_value(&self) -> ExtensionValue {
        ExtensionValue::UseExtendedMasterSecret
    }

    pub fn marshal<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        // length
        writer.write_u16::<BigEndian>(0)?;

        Ok(())
    }

    pub fn unmarshal<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _ = reader.read_u16::<BigEndian>()?;

        Ok(ExtensionUseExtendedMasterSecret { supported: true })
    }
}
