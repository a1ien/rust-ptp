use super::StandardResponseCode;
use std::{fmt, io};

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
            Error::Response(r) => write!(
                f,
                "{} (0x{:04x})",
                StandardResponseCode::name(r).unwrap_or("Unknown"),
                r
            ),
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

    fn cause(&self) -> Option<&dyn (::std::error::Error)> {
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
            io::ErrorKind::UnexpectedEof => Error::Malformed("Unexpected end of message".to_string()),
            _ => Error::Io(e),
        }
    }
}
