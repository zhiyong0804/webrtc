pub mod extension_server_name;
pub mod extension_supported_elliptic_curves;
pub mod extension_supported_point_formats;
pub mod extension_supported_signature_algorithms;
pub mod extension_use_extended_master_secret;
pub mod extension_use_srtp;

use extension_server_name::*;
use extension_supported_elliptic_curves::*;
use extension_supported_point_formats::*;
use extension_supported_signature_algorithms::*;
use extension_use_extended_master_secret::*;
use extension_use_srtp::*;

use crate::errors::*;

use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use util::Error;

// https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml
#[derive(Clone, Debug, PartialEq)]
pub enum ExtensionValue {
    ServerName = 0,
    SupportedEllipticCurves = 10,
    SupportedPointFormats = 11,
    SupportedSignatureAlgorithms = 13,
    UseSRTP = 14,
    UseExtendedMasterSecret = 23,
    Unsupported,
}

impl From<u16> for ExtensionValue {
    fn from(val: u16) -> Self {
        match val {
            0 => ExtensionValue::ServerName,
            10 => ExtensionValue::SupportedEllipticCurves,
            11 => ExtensionValue::SupportedPointFormats,
            13 => ExtensionValue::SupportedSignatureAlgorithms,
            14 => ExtensionValue::UseSRTP,
            23 => ExtensionValue::UseExtendedMasterSecret,
            _ => ExtensionValue::Unsupported,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Extension {
    ServerName(ExtensionServerName),
    SupportedEllipticCurves(ExtensionSupportedEllipticCurves),
    SupportedPointFormats(ExtensionSupportedPointFormats),
    SupportedSignatureAlgorithms(ExtensionSupportedSignatureAlgorithms),
    UseSRTP(ExtensionUseSRTP),
    UseExtendedMasterSecret(ExtensionUseExtendedMasterSecret),
}

impl Extension {
    pub fn extension_value(&self) -> ExtensionValue {
        match self {
            Extension::ServerName(ext) => ext.extension_value(),
            Extension::SupportedEllipticCurves(ext) => ext.extension_value(),
            Extension::SupportedPointFormats(ext) => ext.extension_value(),
            Extension::SupportedSignatureAlgorithms(ext) => ext.extension_value(),
            Extension::UseSRTP(ext) => ext.extension_value(),
            Extension::UseExtendedMasterSecret(ext) => ext.extension_value(),
        }
    }

    pub fn marshal<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self.extension_value() as u16)?;
        match self {
            Extension::ServerName(ext) => ext.marshal(writer),
            Extension::SupportedEllipticCurves(ext) => ext.marshal(writer),
            Extension::SupportedPointFormats(ext) => ext.marshal(writer),
            Extension::SupportedSignatureAlgorithms(ext) => ext.marshal(writer),
            Extension::UseSRTP(ext) => ext.marshal(writer),
            Extension::UseExtendedMasterSecret(ext) => ext.marshal(writer),
        }
    }

    pub fn unmarshal<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let extension_value: ExtensionValue = reader.read_u16::<BigEndian>()?.into();
        match extension_value {
            ExtensionValue::ServerName => Ok(Extension::ServerName(
                ExtensionServerName::unmarshal(reader)?,
            )),
            ExtensionValue::SupportedEllipticCurves => Ok(Extension::SupportedEllipticCurves(
                ExtensionSupportedEllipticCurves::unmarshal(reader)?,
            )),
            ExtensionValue::SupportedPointFormats => Ok(Extension::SupportedPointFormats(
                ExtensionSupportedPointFormats::unmarshal(reader)?,
            )),
            ExtensionValue::SupportedSignatureAlgorithms => {
                Ok(Extension::SupportedSignatureAlgorithms(
                    ExtensionSupportedSignatureAlgorithms::unmarshal(reader)?,
                ))
            }
            ExtensionValue::UseSRTP => Ok(Extension::UseSRTP(ExtensionUseSRTP::unmarshal(reader)?)),
            ExtensionValue::UseExtendedMasterSecret => Ok(Extension::UseExtendedMasterSecret(
                ExtensionUseExtendedMasterSecret::unmarshal(reader)?,
            )),
            _ => Err(ERR_INVALID_EXTENSION_TYPE.clone()),
        }
    }
}
