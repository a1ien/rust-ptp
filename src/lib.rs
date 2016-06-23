#![allow(non_snake_case)]
#[macro_use] extern crate log;

extern crate libusb;
extern crate byteorder;
extern crate time;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, ByteOrder};
use std::io::prelude::*;
use std::io::Cursor;
use std::io;
use std::fmt;
use std::time::Duration;
use std::slice;

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum PtpContainerType {
    Command = 1,
    Data = 2,
    Response = 3,
    Event = 4,
}

impl PtpContainerType {
    fn from_u16(v: u16) -> Option<PtpContainerType> {
        use self::PtpContainerType::*;
        match v {
            1 => Some(Command),
            2 => Some(Data),
            3 => Some(Response),
            4 => Some(Event),
            _ => None
        }
    }
}

pub type ResponseCode = u16;

#[allow(non_upper_case_globals)]
pub mod StandardResponseCode {
    pub type Value = super::ResponseCode;
    
    pub const Undefined: Value = 0x2000;
    pub const Ok: Value = 0x2001;
    pub const GeneralError: Value = 0x2002;
    pub const SessionNotOpen: Value = 0x2003;
    pub const InvalidTransactionId: Value = 0x2004;
    pub const OperationNotSupported: Value = 0x2005;
    pub const ParameterNotSupported: Value = 0x2006;
    pub const IncompleteTransfer: Value = 0x2007;
    pub const InvalidStorageId: Value = 0x2008;
    pub const InvalidObjectHandle: Value = 0x2009;
    pub const DevicePropNotSupported: Value = 0x200A;
    pub const InvalidObjectFormatCode: Value = 0x200B;
    pub const StoreFull: Value = 0x200C;
    pub const ObjectWriteProtected: Value = 0x200D;
    pub const StoreReadOnly: Value = 0x200E;
    pub const AccessDenied: Value = 0x200F;
    pub const NoThumbnailPresent: Value = 0x2010;
    pub const SelfTestFailed: Value = 0x2011;
    pub const PartialDeletion: Value = 0x2012;
    pub const StoreNotAvailable: Value = 0x2013;
    pub const SpecificationByFormatUnsupported: Value = 0x2014;
    pub const NoValidObjectInfo: Value = 0x2015;
    pub const InvalidCodeFormat: Value = 0x2016;
    pub const UnknownVendorCode: Value = 0x2017;
    pub const CaptureAlreadyTerminated: Value = 0x2018;
    pub const DeviceBusy: Value = 0x2019;
    pub const InvalidParentObject: Value = 0x201A;
    pub const InvalidDevicePropFormat: Value = 0x201B;
    pub const InvalidDevicePropValue: Value = 0x201C;
    pub const InvalidParameter: Value = 0x201D;
    pub const SessionAlreadyOpen: Value = 0x201E;
    pub const TransactionCancelled: Value = 0x201F;
    pub const SpecificationOfDestinationUnsupported: Value = 0x2020;
    
    pub fn name(v: Value) -> Option<&'static str> {
        match v {
            Undefined => Some("Undefined"),
            Ok => Some("Ok"),
            GeneralError => Some("GeneralError"),
            SessionNotOpen => Some("SessionNotOpen"),
            InvalidTransactionId => Some("InvalidTransactionId"),
            OperationNotSupported => Some("OperationNotSupported"),
            ParameterNotSupported => Some("ParameterNotSupported"),
            IncompleteTransfer => Some("IncompleteTransfer"),
            InvalidStorageId => Some("InvalidStorageId"),
            InvalidObjectHandle => Some("InvalidObjectHandle"),
            DevicePropNotSupported => Some("DevicePropNotSupported"),
            InvalidObjectFormatCode => Some("InvalidObjectFormatCode"),
            StoreFull => Some("StoreFull"),
            ObjectWriteProtected => Some("ObjectWriteProtected"),
            StoreReadOnly => Some("StoreReadOnly"),
            AccessDenied => Some("AccessDenied"),
            NoThumbnailPresent => Some("NoThumbnailPresent"),
            SelfTestFailed => Some("SelfTestFailed"),
            PartialDeletion => Some("PartialDeletion"),
            StoreNotAvailable => Some("StoreNotAvailable"),
            SpecificationByFormatUnsupported => Some("SpecificationByFormatUnsupported"),
            NoValidObjectInfo => Some("NoValidObjectInfo"),
            InvalidCodeFormat => Some("InvalidCodeFormat"),
            UnknownVendorCode => Some("UnknownVendorCode"),
            CaptureAlreadyTerminated => Some("CaptureAlreadyTerminated"),
            DeviceBusy => Some("DeviceBusy"),
            InvalidParentObject => Some("InvalidParentObject"),
            InvalidDevicePropFormat => Some("InvalidDevicePropFormat"),
            InvalidDevicePropValue => Some("InvalidDevicePropValue"),
            InvalidParameter => Some("InvalidParameter"),
            SessionAlreadyOpen => Some("SessionAlreadyOpen"),
            TransactionCancelled => Some("TransactionCancelled"),
            SpecificationOfDestinationUnsupported => Some("SpecificationOfDestinationUnsupported"),
            _ => None,
        }
    }
}

pub type CommandCode = u16;

#[allow(non_upper_case_globals)]
pub mod StandardCommandCode {
    pub type Value = super::CommandCode;
    
    pub const Undefined: Value = 0x1000;
    pub const GetDeviceInfo: Value = 0x1001;
    pub const OpenSession: Value = 0x1002;
    pub const CloseSession: Value = 0x1003;
    pub const GetStorageIDs: Value = 0x1004;
    pub const GetStorageInfo: Value = 0x1005;
    pub const GetNumObjects: Value = 0x1006;
    pub const GetObjectHandles: Value = 0x1007;
    pub const GetObjectInfo: Value = 0x1008;
    pub const GetObject: Value = 0x1009;
    pub const GetThumb: Value = 0x100A;
    pub const DeleteObject: Value = 0x100B;
    pub const SendObjectInfo: Value = 0x100C;
    pub const SendObject: Value = 0x100D;
    pub const InitiateCapture: Value = 0x100E;
    pub const FormatStore: Value = 0x100F;
    pub const ResetDevice: Value = 0x1010;
    pub const SelfTest: Value = 0x1011;
    pub const SetObjectProtection: Value = 0x1012;
    pub const PowerDown: Value = 0x1013;
    pub const GetDevicePropDesc: Value = 0x1014;
    pub const GetDevicePropValue: Value = 0x1015;
    pub const SetDevicePropValue: Value = 0x1016;
    pub const ResetDevicePropValue: Value = 0x1017;
    pub const TerminateOpenCapture: Value = 0x1018;
    pub const MoveObject: Value = 0x1019;
    pub const CopyObject: Value = 0x101A;
    pub const GetPartialObject: Value = 0x101B;
    pub const InitiateOpenCapture: Value = 0x101C;
    
    pub fn name(v: Value) -> Option<&'static str> {
        match v {
            Undefined => Some("Undefined"),
            GetDeviceInfo => Some("GetDeviceInfo"),
            OpenSession => Some("OpenSession"),
            CloseSession => Some("CloseSession"),
            GetStorageIDs => Some("GetStorageIDs"),
            GetStorageInfo => Some("GetStorageInfo"),
            GetNumObjects => Some("GetNumObjects"),
            GetObjectHandles => Some("GetObjectHandles"),
            GetObjectInfo => Some("GetObjectInfo"),
            GetObject => Some("GetObject"),
            GetThumb => Some("GetThumb"),
            DeleteObject => Some("DeleteObject"),
            SendObjectInfo => Some("SendObjectInfo"),
            SendObject => Some("SendObject"),
            InitiateCapture => Some("InitiateCapture"),
            FormatStore => Some("FormatStore"),
            ResetDevice => Some("ResetDevice"),
            SelfTest => Some("SelfTest"),
            SetObjectProtection => Some("SetObjectProtection"),
            PowerDown => Some("PowerDown"),
            GetDevicePropDesc => Some("GetDevicePropDesc"),
            GetDevicePropValue => Some("GetDevicePropValue"),
            SetDevicePropValue => Some("SetDevicePropValue"),
            ResetDevicePropValue => Some("ResetDevicePropValue"),
            TerminateOpenCapture => Some("TerminateOpenCapture"),
            MoveObject => Some("MoveObject"),
            CopyObject => Some("CopyObject"),
            GetPartialObject => Some("GetPartialObject"),
            InitiateOpenCapture => Some("InitiateOpenCapture"),
            _ => None,
        }
    }
}

/// An error in a PTP command
#[derive(Debug)]
pub enum Error {
    /// PTP Responder returned a status code other than Ok, either a constant in StandardResponseCode or a vendor-defined code
    Response(u16),
    
    /// Data received was malformed
    Malformed(String),
    
    /// Another libusb error
    Usb(libusb::Error),
    
    /// Another IO error
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Response(r) => write!(f, "{} (0x{:04x})", StandardResponseCode::name(r).unwrap_or("Unknown"), r),
            Error::Usb(ref e) => write!(f, "USB error: {}", e),
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Malformed(ref e) => write!(f, "{}", e),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Response(r) => StandardResponseCode::name(r).unwrap_or("<vendor-defined code>"),
            Error::Malformed(ref m) => m,
            Error::Usb(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Usb(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<libusb::Error> for Error {
    fn from(e: libusb::Error) -> Error {
        Error::Usb(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => Error::Malformed(format!("Unexpected end of message")),
            _ => Error::Io(e),
        }
    }
}

pub trait PtpRead: ReadBytesExt {
    fn read_ptp_u8(&mut self) -> Result<u8, Error> {
        Ok(try!(self.read_u8()))
    }

    fn read_ptp_i8(&mut self) -> Result<i8, Error> {
        Ok(try!(self.read_i8()))
    }

    fn read_ptp_u16(&mut self) -> Result<u16, Error> {
        Ok(try!(self.read_u16::<LittleEndian>()))
    }

    fn read_ptp_i16(&mut self) -> Result<i16, Error> {
        Ok(try!(self.read_i16::<LittleEndian>()))
    }

    fn read_ptp_u32(&mut self) -> Result<u32, Error> {
        Ok(try!(self.read_u32::<LittleEndian>()))
    }

    fn read_ptp_i32(&mut self) -> Result<i32, Error> {
        Ok(try!(self.read_i32::<LittleEndian>()))
    }

    fn read_ptp_u64(&mut self) -> Result<u64, Error> {
        Ok(try!(self.read_u64::<LittleEndian>()))
    }

    fn read_ptp_i64(&mut self) -> Result<i64, Error> {
        Ok(try!(self.read_i64::<LittleEndian>()))
    }

    fn read_ptp_u128(&mut self) -> Result<(u64, u64), Error> {
        let hi = try!(self.read_u64::<LittleEndian>());
        let lo = try!(self.read_u64::<LittleEndian>());
        Ok((lo, hi))
    }

    fn read_ptp_i128(&mut self) -> Result<(u64, u64), Error> {
        let hi = try!(self.read_u64::<LittleEndian>());
        let lo = try!(self.read_u64::<LittleEndian>());
        Ok((lo, hi))
    }

    #[inline(always)]
    fn read_ptp_vec<T: Sized, U: Fn(&mut Self) -> Result<T, Error>>(&mut self,
                                                                 func: U)
                                                                 -> Result<Vec<T>, Error> {
        let len = try!(self.read_u32::<LittleEndian>()) as usize;
        (0..len).map(|_| func(self)).collect()
    }

    fn read_ptp_u8_vec(&mut self) -> Result<Vec<u8>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_u8())
    }

    fn read_ptp_i8_vec(&mut self) -> Result<Vec<i8>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_i8())
    }

    fn read_ptp_u16_vec(&mut self) -> Result<Vec<u16>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_u16())
    }

    fn read_ptp_i16_vec(&mut self) -> Result<Vec<i16>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_i16())
    }

    fn read_ptp_u32_vec(&mut self) -> Result<Vec<u32>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_u32())
    }

    fn read_ptp_i32_vec(&mut self) -> Result<Vec<i32>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_i32())
    }

    fn read_ptp_u64_vec(&mut self) -> Result<Vec<u64>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_u64())
    }

    fn read_ptp_i64_vec(&mut self) -> Result<Vec<i64>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_i64())
    }

    fn read_ptp_u128_vec(&mut self) -> Result<Vec<(u64, u64)>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_u128())
    }

    fn read_ptp_i128_vec(&mut self) -> Result<Vec<(u64, u64)>, Error> {
        self.read_ptp_vec(|cur| cur.read_ptp_i128())
    }

    fn read_ptp_str(&mut self) -> Result<String, Error> {
        let len = try!(self.read_u8());
        if len > 0 {
            // len includes the trailing null u16
            let data: Vec<u16> = try!((0..(len - 1)).map(|_| self.read_u16::<LittleEndian>()).collect());
            try!(self.read_u16::<LittleEndian>());
            String::from_utf16(&data).map_err(|_| Error::Malformed(format!("Invalid UTF16 data: {:?}", data)))
        } else {
            Ok("".into())
        }
    }
    
    fn expect_end(&mut self) -> Result<(), Error>;
}

impl<T: AsRef<[u8]>> PtpRead for Cursor<T> {
    fn expect_end(&mut self) -> Result<(), Error> {
        let len = self.get_ref().as_ref().len();
        if len as u64 != self.position() {
            Err(Error::Malformed(format!("Response {} bytes, expected {} bytes", len, self.position())))
        } else {
            Ok(())
        }
    }
}


#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub enum PtpDataType {
    UNDEF,
    INT8(i8),
    UINT8(u8),
    INT16(i16),
    UINT16(u16),
    INT32(i32),
    UINT32(u32),
    INT64(i64),
    UINT64(u64),
    INT128((u64, u64)),
    UINT128((u64, u64)),
    AINT8(Vec<i8>),
    AUINT8(Vec<u8>),
    AINT16(Vec<i16>),
    AUINT16(Vec<u16>),
    AINT32(Vec<i32>),
    AUINT32(Vec<u32>),
    AINT64(Vec<i64>),
    AUINT64(Vec<u64>),
    AINT128(Vec<(u64, u64)>),
    AUINT128(Vec<(u64, u64)>),
    STR(String),
}

impl PtpDataType {
    pub fn encode(&self) -> Vec<u8> {
        use self::PtpDataType::*;
        let mut out = vec![];
        match self {
            // UNDEF => {},
            &INT8(val) => {
                out.write_i8(val).ok();
            }
            &UINT8(val) => {
                out.write_u8(val).ok();
            }
            &INT16(val) => {
                out.write_i16::<LittleEndian>(val).ok();
            }
            &UINT16(val) => {
                out.write_u16::<LittleEndian>(val).ok();
            }
            &INT32(val) => {
                out.write_i32::<LittleEndian>(val).ok();
            }
            &UINT32(val) => {
                out.write_u32::<LittleEndian>(val).ok();
            }
            &INT64(val) => {
                out.write_i64::<LittleEndian>(val).ok();
            }
            &UINT64(val) => {
                out.write_u64::<LittleEndian>(val).ok();
            }
            &INT128((hi, lo)) => {
                out.write_u64::<LittleEndian>(lo).ok();
                out.write_u64::<LittleEndian>(hi).ok();
            }
            &UINT128((hi, lo)) => {
                out.write_u64::<LittleEndian>(lo).ok();
                out.write_u64::<LittleEndian>(hi).ok();
            }
            &AINT8(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_i8(*item).ok();
                }
            }
            &AUINT8(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_u8(*item).ok();
                }
            }
            &AINT16(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_i16::<LittleEndian>(*item).ok();
                }
            }
            &AUINT16(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_u16::<LittleEndian>(*item).ok();
                }
            }
            &AINT32(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_i32::<LittleEndian>(*item).ok();
                }
            }
            &AUINT32(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_u32::<LittleEndian>(*item).ok();
                }
            }
            &AINT64(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_i64::<LittleEndian>(*item).ok();
                }
            }
            &AUINT64(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for item in val {
                    out.write_u64::<LittleEndian>(*item).ok();
                }
            }
            &AINT128(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for &(hi, lo) in val {
                    out.write_u64::<LittleEndian>(lo).ok();
                    out.write_u64::<LittleEndian>(hi).ok();
                }
            }
            &AUINT128(ref val) => {
                out.write_u32::<LittleEndian>(val.len() as u32).ok();
                for &(hi, lo) in val {
                    out.write_u64::<LittleEndian>(lo).ok();
                    out.write_u64::<LittleEndian>(hi).ok();
                }
            }
            &STR(ref val) => {
                out.write_u8(((val.len() as u8) * 2) + 1).ok();
                if val.len() > 0 {
                    for e in val.encode_utf16() { out.write_u16::<LittleEndian>(e).ok(); }
                    out.write_all(b"\0\0").ok();
                }
            }
            _ => {}
        }
        out
    }

    pub fn read_type<T: PtpRead>(kind: u16, reader: &mut T) -> Result<PtpDataType, Error> {
        use self::PtpDataType::*;
        Ok(match kind {
            // 0x0000 => UNDEF,
            0x0001 => INT8(try!(reader.read_ptp_i8())),
            0x0002 => UINT8(try!(reader.read_ptp_u8())),
            0x0003 => INT16(try!(reader.read_ptp_i16())),
            0x0004 => UINT16(try!(reader.read_ptp_u16())),
            0x0005 => INT32(try!(reader.read_ptp_i32())),
            0x0006 => UINT32(try!(reader.read_ptp_u32())),
            0x0007 => INT64(try!(reader.read_ptp_i64())),
            0x0008 => UINT64(try!(reader.read_ptp_u64())),
            0x0009 => INT128(try!(reader.read_ptp_i128())),
            0x000A => UINT128(try!(reader.read_ptp_u128())),
            0x4001 => AINT8(try!(reader.read_ptp_i8_vec())),
            0x4002 => AUINT8(try!(reader.read_ptp_u8_vec())),
            0x4003 => AINT16(try!(reader.read_ptp_i16_vec())),
            0x4004 => AUINT16(try!(reader.read_ptp_u16_vec())),
            0x4005 => AINT32(try!(reader.read_ptp_i32_vec())),
            0x4006 => AUINT32(try!(reader.read_ptp_u32_vec())),
            0x4007 => AINT64(try!(reader.read_ptp_i64_vec())),
            0x4008 => AUINT64(try!(reader.read_ptp_u64_vec())),
            0x4009 => AINT128(try!(reader.read_ptp_i128_vec())),
            0x400A => AUINT128(try!(reader.read_ptp_u128_vec())),
            0xFFFF => STR(try!(reader.read_ptp_str())),
            _ => UNDEF,
        })
    }
}

impl<'a> From<i8> for PtpDataType {
    fn from(value: i8) -> Self {
        PtpDataType::INT8(value)
    }
}

impl<'a> From<u8> for PtpDataType {
    fn from(value: u8) -> Self {
        PtpDataType::UINT8(value)
    }
}

impl<'a> From<i16> for PtpDataType {
    fn from(value: i16) -> Self {
        PtpDataType::INT16(value)
    }
}

impl<'a> From<u16> for PtpDataType {
    fn from(value: u16) -> Self {
        PtpDataType::UINT16(value)
    }
}

impl<'a> From<i32> for PtpDataType {
    fn from(value: i32) -> Self {
        PtpDataType::INT32(value)
    }
}

impl<'a> From<u32> for PtpDataType {
    fn from(value: u32) -> Self {
        PtpDataType::UINT32(value)
    }
}

impl<'a> From<i64> for PtpDataType {
    fn from(value: i64) -> Self {
        PtpDataType::INT64(value)
    }
}

impl<'a> From<u64> for PtpDataType {
    fn from(value: u64) -> Self {
        PtpDataType::UINT64(value)
    }
}

impl<'a> From<&'a str> for PtpDataType {
    fn from(value: &'a str) -> Self {
        PtpDataType::STR(value.to_owned())
    }
}

impl<'a> From<String> for PtpDataType {
    fn from(value: String) -> Self {
        PtpDataType::STR(value)
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct PtpDeviceInfo {
    pub Version: u16,
    pub VendorExID: u32,
    pub VendorExVersion: u16,
    pub VendorExtensionDesc: String,
    pub FunctionalMode: u16,
    pub OperationsSupported: Vec<u16>,
    pub EventsSupported: Vec<u16>,
    pub DevicePropertiesSupported: Vec<u16>,
    pub CaptureFormats: Vec<u16>,
    pub ImageFormats: Vec<u16>,
    pub Manufacturer: String,
    pub Model: String,
    pub DeviceVersion: String,
    pub SerialNumber: String,
}

impl PtpDeviceInfo {
    pub fn decode(buf: &[u8]) -> Result<PtpDeviceInfo, Error> {
        let mut cur = Cursor::new(buf);

        Ok(PtpDeviceInfo {
            Version: try!(cur.read_ptp_u16()),
            VendorExID: try!(cur.read_ptp_u32()),
            VendorExVersion: try!(cur.read_ptp_u16()),
            VendorExtensionDesc: try!(cur.read_ptp_str()),
            FunctionalMode: try!(cur.read_ptp_u16()),
            OperationsSupported: try!(cur.read_ptp_u16_vec()),
            EventsSupported: try!(cur.read_ptp_u16_vec()),
            DevicePropertiesSupported: try!(cur.read_ptp_u16_vec()),
            CaptureFormats: try!(cur.read_ptp_u16_vec()),
            ImageFormats: try!(cur.read_ptp_u16_vec()),
            Manufacturer: try!(cur.read_ptp_str()),
            Model: try!(cur.read_ptp_str()),
            DeviceVersion: try!(cur.read_ptp_str()),
            SerialNumber: try!(cur.read_ptp_str()),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PtpObjectInfo {
    pub StorageID: u32,
    pub ObjectFormat: u16,
    pub ProtectionStatus: u16,
    pub ObjectCompressedSize: u32,
    pub ThumbFormat: u16,
    pub ThumbCompressedSize: u32,
    pub ThumbPixWidth: u32,
    pub ThumbPixHeight: u32,
    pub ImagePixWidth: u32,
    pub ImagePixHeight: u32,
    pub ImageBitDepth: u32,
    pub ParentObject: u32,
    pub AssociationType: u16,
    pub AssociationDesc: u32,
    pub SequenceNumber: u32,
    pub Filename: String,
    pub CaptureDate: String,
    pub ModificationDate: String,
    pub Keywords: String,
}

impl PtpObjectInfo {
    pub fn decode(buf: &[u8]) -> Result<PtpObjectInfo, Error> {
        let mut cur = Cursor::new(buf);

        Ok(PtpObjectInfo {
            StorageID: try!(cur.read_ptp_u32()),
            ObjectFormat: try!(cur.read_ptp_u16()),
            ProtectionStatus: try!(cur.read_ptp_u16()),
            ObjectCompressedSize: try!(cur.read_ptp_u32()),
            ThumbFormat: try!(cur.read_ptp_u16()),
            ThumbCompressedSize: try!(cur.read_ptp_u32()),
            ThumbPixWidth: try!(cur.read_ptp_u32()),
            ThumbPixHeight: try!(cur.read_ptp_u32()),
            ImagePixWidth: try!(cur.read_ptp_u32()),
            ImagePixHeight: try!(cur.read_ptp_u32()),
            ImageBitDepth: try!(cur.read_ptp_u32()),
            ParentObject: try!(cur.read_ptp_u32()),
            AssociationType: try!(cur.read_ptp_u16()),
            AssociationDesc: try!(cur.read_ptp_u32()),
            SequenceNumber: try!(cur.read_ptp_u32()),
            Filename: try!(cur.read_ptp_str()),
            CaptureDate: try!(cur.read_ptp_str()),
            ModificationDate: try!(cur.read_ptp_str()),
            Keywords: try!(cur.read_ptp_str()),
        })
    }
}


#[allow(non_snake_case)]
#[derive(Debug)]
pub struct PtpStorageInfo {
    pub StorageType: u16,
    pub FilesystemType: u16,
    pub AccessCapability: u16,
    pub MaxCapacity: u64,
    pub FreeSpaceInBytes: u64,
    pub FreeSpaceInImages: u32,
    pub StorageDescription: String,
    pub VolumeLabel: String,
}

impl PtpStorageInfo {
    pub fn decode<T: PtpRead>(cur: &mut T) -> Result<PtpStorageInfo, Error> {
        Ok(PtpStorageInfo {
            StorageType: try!(cur.read_ptp_u16()),
            FilesystemType: try!(cur.read_ptp_u16()),
            AccessCapability: try!(cur.read_ptp_u16()),
            MaxCapacity: try!(cur.read_ptp_u64()),
            FreeSpaceInBytes: try!(cur.read_ptp_u64()),
            FreeSpaceInImages: try!(cur.read_ptp_u32()),
            StorageDescription: try!(cur.read_ptp_str()),
            VolumeLabel: try!(cur.read_ptp_str()),
        })
    }
}



#[allow(non_snake_case)]
#[derive(Debug)]
pub enum PtpFormData {
    None,
    Range {
        minValue: PtpDataType,
        maxValue: PtpDataType,
        step: PtpDataType,
    },
    Enumeration {
        array: Vec<PtpDataType>,
    },
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct PtpPropInfo {
    pub PropertyCode: u16,
    pub DataType: u16,
    pub GetSet: u8,
    pub IsEnable: u8,
    pub FactoryDefault: PtpDataType,
    pub Current: PtpDataType,
    pub Form: PtpFormData,
}

impl PtpPropInfo {
    pub fn decode<T: PtpRead>(cur: &mut T) -> Result<PtpPropInfo, Error> {
        let data_type;
        Ok(PtpPropInfo {
            PropertyCode: try!(cur.read_u16::<LittleEndian>()),
            DataType: {
                data_type = try!(cur.read_u16::<LittleEndian>());
                data_type
            },
            GetSet: try!(cur.read_u8()),
            IsEnable: try!(cur.read_u8()),
            FactoryDefault: try!(PtpDataType::read_type(data_type, cur)),
            Current: try!(PtpDataType::read_type(data_type, cur)),
            Form: {
                match try!(cur.read_u8()) {
                    // 0x00 => PtpFormData::None,
                    0x01 => {
                        PtpFormData::Range {
                            minValue: try!(PtpDataType::read_type(data_type, cur)),
                            maxValue: try!(PtpDataType::read_type(data_type, cur)),
                            step: try!(PtpDataType::read_type(data_type, cur)),
                        }
                    }
                    0x02 => {
                        PtpFormData::Enumeration {
                            array: {
                                let len = try!(cur.read_u16::<LittleEndian>()) as usize;
                                let mut arr = Vec::with_capacity(len);
                                for _ in 0..len {
                                    arr.push(try!(PtpDataType::read_type(data_type, cur)));
                                }
                                arr
                            },
                        }
                    }
                    _ => PtpFormData::None,
                }
            },
        })
    }
}

#[derive(Debug)]
struct PtpTransaction {
    pub tid: u32,
    pub code: u16,
    pub data: Vec<u8>,
}

impl PtpTransaction {
    pub fn parse(buf: &[u8]) -> Result<(PtpContainerType, PtpTransaction), Error> {
        let mut cur = Cursor::new(buf);

        let len = try!(cur.read_u32::<LittleEndian>());

        let msgtype = try!(cur.read_u16::<LittleEndian>());
        let mtype = try!(PtpContainerType::from_u16(msgtype)
            .ok_or_else(|| Error::Malformed(format!("Invalid message type {:x}.", msgtype))));
        let code = try!(cur.read_u16::<LittleEndian>());
        let tid = try!(cur.read_u32::<LittleEndian>());

        let data_len = if len > 12 {
            len - 12
        } else {
            0
        };
        let mut data = Vec::with_capacity(data_len as usize);
        try!(cur.read_to_end(&mut data));

        Ok((mtype,
            PtpTransaction {
            tid: tid,
            code: code,
            data: data,
        }))
    }

    pub fn is_response(&self, target: &PtpTransaction) -> bool {
        self.tid == target.tid
    }
}

fn ptp_gen_message(w: &mut Write,
                   kind: PtpContainerType,
                   code: CommandCode,
                   tid: u32,
                   payload: &[u8]) {
    let len: u32 = 12 + payload.len() as u32;

    w.write_u32::<LittleEndian>(len).ok();
    w.write_u16::<LittleEndian>(kind as u16).ok();
    w.write_u16::<LittleEndian>(code).ok();
    w.write_u32::<LittleEndian>(tid).ok();
    w.write_all(payload).ok();
}

fn ptp_gen_cmd_message(w: &mut Write, code: CommandCode, tid: u32, params: &[u32]) {
    let mut payload = vec![];
    for p in params {
        payload.write_u32::<LittleEndian>(*p).ok();
    }
    ptp_gen_message(w, PtpContainerType::Command, code, tid, &payload);
}

pub struct PtpCamera<'a> {
    buf: Vec<u8>, // TODO make this private
    iface: u8,
    ep_in: u8,
    ep_out: u8,
    _ep_int: u8,
    current_tid: u32,
    handle: libusb::DeviceHandle<'a>,
}

impl<'a> PtpCamera<'a> {
    pub fn new(device: &mut libusb::Device, mut handle: libusb::DeviceHandle<'a>) -> Result<PtpCamera<'a>, Error> {
        // TODO: handle non-default configurations once https://github.com/dcuddeback/libusb-rs/pull/9 is released
        let config_desc = try!(device.config_descriptor(0));
        
        let interface_desc = try!(config_desc.interfaces()
            .flat_map(|i| i.descriptors())
            .find(|x| x.class_code() == 6)
            .ok_or(libusb::Error::NotFound));
            
        debug!("Found interface {}", interface_desc.interface_number());

        try!(handle.claim_interface(interface_desc.interface_number()));
        try!(handle.set_alternate_setting(interface_desc.interface_number(), interface_desc.setting_number()));
        
        let find_endpoint = |direction, transfer_type| {
            interface_desc.endpoint_descriptors()
                .find(|ep| ep.direction() == direction && ep.transfer_type() == transfer_type)
                .map(|x| x.address())
                .ok_or(libusb::Error::NotFound)
        };

        Ok(PtpCamera {
            buf: Vec::with_capacity(1*1024*1024),
            iface: interface_desc.interface_number(),
            ep_in:  try!(find_endpoint(libusb::Direction::In, libusb::TransferType::Bulk)),
            ep_out: try!(find_endpoint(libusb::Direction::Out, libusb::TransferType::Bulk)),
            _ep_int: try!(find_endpoint(libusb::Direction::In, libusb::TransferType::Interrupt)),
            current_tid: 0,
            handle: handle,
        })
    }
    
    pub fn command(&mut self,
                   code: CommandCode,
                   params: &[u32],
                   data: Option<&[u8]>)
                   -> Result<Vec<u8>, Error> {
                                          
        let transaction = PtpTransaction {
            tid: self.current_tid,
            code: code,
            data: vec![], // TODO
        };

        let timeout = Duration::from_secs(2);

        // Send messages.
        let mut cmd_message = vec![];
        ptp_gen_cmd_message(&mut cmd_message, code, self.current_tid, params);

        let timespec = time::get_time();
        trace!("Write Cmnd [{}:{:09}] - 0x{:04x} ({}), tid:{}, params:{:?}",
               timespec.sec,
               timespec.nsec,
               code,
               StandardCommandCode::name(code).unwrap_or("unknown"),
               self.current_tid,
               params);
        
        try!(self.handle.write_bulk(self.ep_out, &cmd_message, timeout));

        if let Some(data) = data {
            let mut data_message = vec![];
            ptp_gen_message(&mut data_message,
                            PtpContainerType::Data,
                            code,
                            self.current_tid,
                            data);
            let timespec = time::get_time();
            trace!("Write Data [{}:{:09}] - 0x{:04x} ({}), tid:{}, len:{}",
                   timespec.sec,
                   timespec.nsec,
                   code,
                   StandardCommandCode::name(code).unwrap_or("unknown"),
                   self.current_tid,
                   data.len());
            try!(self.handle.write_bulk(self.ep_out, &data_message, timeout));
        }

        self.current_tid += 1;

        let mut data = None;
        loop {
            unsafe {
                self.buf.set_len(0);
            }

            loop {
                let chunk_size = 256 * 1024;
                let current_len = self.buf.len();
                let current_capacity = self.buf.capacity();
                if current_capacity - current_len < chunk_size {
                    self.buf.reserve(chunk_size);
                }
                let remaining_buf = unsafe {
                    slice::from_raw_parts_mut(self.buf.get_unchecked_mut(current_len) as *mut _, chunk_size)
                };
                let timespec = time::get_time();
                trace!("Read Data  [{}:{:09}] - length:{:?} remaining:{:?}",
                       timespec.sec,
                       timespec.nsec,
                       current_len,
                       remaining_buf.len());
                let len = try!(self.handle.read_bulk(self.ep_in, remaining_buf, timeout));
                unsafe {
                    self.buf.set_len(current_len + len);
                }
                // debug!("new buf len [{:?}] into {:?}", self.buf.len(), remaining_buf.len());
                if len < remaining_buf.len() {
                    break;
                }
            }

            let (mtype, mut msg) = try!(PtpTransaction::parse(&self.buf));

            if mtype == PtpContainerType::Data && msg.is_response(&transaction) {
                data = Some(msg.data);
            } else if mtype == PtpContainerType::Response && msg.is_response(&transaction) {
                if let Some(data) = data {
                    msg.data = data;
                }
                
                if msg.code != StandardResponseCode::Ok {
                    return Err(Error::Response(msg.code));
                }
                
                return Ok(msg.data);
            }
        }
    }

    pub fn get_objectinfo(&mut self, handle: u32) -> Result<PtpObjectInfo, Error> {
        let data = try!(self.command(StandardCommandCode::GetObjectInfo, &vec![handle], None));
        Ok(try!(PtpObjectInfo::decode(&data)))
    }

    pub fn get_object(&mut self, handle: u32) -> Result<Vec<u8>, Error> {
        self.command(StandardCommandCode::GetObject, &vec![handle], None)
    }

    pub fn get_objecthandles(&mut self,
                             storage_id: u32,
                             handle_id: u32,
                             filter: Option<u32>)
                             -> Result<Vec<u32>, Error> {
        let data = try!(self.command(StandardCommandCode::GetObjectHandles,
                                    &[storage_id, filter.unwrap_or(0x0), handle_id],
                                    None));
        // Parse ObjectHandleArrray
        let mut cur = Cursor::new(data);
        let value = try!(cur.read_ptp_u32_vec());
        try!(cur.expect_end());
        
        Ok(value)
    }

    pub fn get_objecthandles_root(&mut self,
                                  storage_id: u32,
                                  filter: Option<u32>)
                                  -> Result<Vec<u32>, Error> {
        self.get_objecthandles(storage_id, 0xFFFFFFFF, filter)
    }

    pub fn get_objecthandles_all(&mut self,
                                 storage_id: u32,
                                 filter: Option<u32>)
                                 -> Result<Vec<u32>, Error> {
        self.get_objecthandles(storage_id, 0x0, filter)
    }

    // handle_id: None == root of store
    pub fn get_numobjects(&mut self,
                          storage_id: u32,
                          handle_id: u32,
                          filter: Option<u32>)
                          -> Result<u32, Error> {
        let data = try!(self.command(StandardCommandCode::GetNumObjects,
                                    &[storage_id, filter.unwrap_or(0x0), handle_id],
                                    None));

        // Parse ObjectHandleArrray
        let mut cur = Cursor::new(data);
        let value = try!(cur.read_ptp_u32());
        try!(cur.expect_end());

        Ok(value)
    }

    pub fn get_storage_info(&mut self, storage_id: u32) -> Result<PtpStorageInfo, Error> {
        let data = try!(self.command(StandardCommandCode::GetStorageInfo, &[storage_id], None));

        // Parse ObjectHandleArrray
        let mut cur = Cursor::new(data);
        let res = try!(PtpStorageInfo::decode(&mut cur));
        try!(cur.expect_end());

        Ok(res)
    }

    pub fn get_storageids(&mut self) -> Result<Vec<u32>, Error> {
        let data = try!(self.command(StandardCommandCode::GetStorageIDs, &[], None));

        // Parse ObjectHandleArrray
        let mut cur = Cursor::new(data);
        let value = try!(cur.read_ptp_u32_vec());
        try!(cur.expect_end());

        Ok(value)
    }

    pub fn get_numobjects_roots(&mut self,
                                storage_id: u32,
                                filter: Option<u32>)
                                -> Result<u32, Error> {
        self.get_numobjects(storage_id, 0xFFFFFFFF, filter)
    }

    pub fn get_numobjects_all(&mut self, storage_id: u32, filter: Option<u32>) -> Result<u32, Error> {
        self.get_numobjects(storage_id, 0x0, filter)
    }

    pub fn get_device_info(&mut self) -> Result<PtpDeviceInfo, Error> {
        let data = try!(self.command(StandardCommandCode::GetDeviceInfo, &vec![0, 0, 0], None));

        let device_info = try!(PtpDeviceInfo::decode(&data));
        debug!("device_info {:?}", device_info);
        Ok(device_info)
    }

    pub fn open_session(&mut self) -> Result<(), Error> {
        let session_id = 3;

        try!(self.command(StandardCommandCode::OpenSession,
                     &vec![session_id, 0, 0],
                     None));

        Ok(())
    }

    pub fn close_session(&mut self) -> Result<(), Error> {
        try!(self.command(StandardCommandCode::CloseSession, &vec![], None));
        
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), Error> {
        try!(self.close_session());
        try!(self.handle.release_interface(self.iface));
        Ok(())
    }
}

pub fn open_device(context: &mut libusb::Context,
                   vid: u16,
                   pid: u16)
                   -> Option<(libusb::Device, libusb::DeviceDescriptor, libusb::DeviceHandle)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for mut device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct PtpObjectTree {
    pub handle: u32,
    pub info: PtpObjectInfo,
    pub children: Option<Vec<PtpObjectTree>>,
}

impl PtpObjectTree {
    pub fn walk(&self) -> Vec<(String, PtpObjectTree)> {
        let mut input = vec![("".to_owned(), self.clone())];
        let mut output = vec![];

        while !input.is_empty() {
            for (prefix, item) in input.split_off(0) {
                let path = prefix.clone() +
                           (if prefix.is_empty() {
                    ""
                } else {
                    "/"
                }) + &item.info.Filename;

                output.push((path.clone(), item.clone()));

                if let Some(children) = item.children {
                    input.extend(children.into_iter().map(|x| (path.clone(), x)));
                }
            }
        }

        output
    }
}
