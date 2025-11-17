//! Extensions specific to the HTTP/2 protocol.

use crate::hpack::BytesStr;

use bytes::Bytes;
use http::{uri, Method};
use std::fmt;

/// Represents the `:protocol` pseudo-header used by
/// the [Extended CONNECT Protocol].
///
/// [Extended CONNECT Protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
#[derive(Clone, Eq, PartialEq)]
pub struct Protocol {
    value: BytesStr,
}

impl Protocol {
    /// Converts a static string to a protocol name.
    pub const fn from_static(value: &'static str) -> Self {
        Self {
            value: BytesStr::from_static(value),
        }
    }

    /// Returns a str representation of the header.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub(crate) fn try_from(bytes: Bytes) -> Result<Self, std::str::Utf8Error> {
        Ok(Self {
            value: BytesStr::try_from(bytes)?,
        })
    }
}

impl<'a> From<&'a str> for Protocol {
    fn from(value: &'a str) -> Self {
        Self {
            value: BytesStr::from(value),
        }
    }
}

impl AsRef<[u8]> for Protocol {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Allows overriding the request pseudo headers before a `Request` is encoded.
#[derive(Clone, Debug, Default)]
pub struct PseudoHeadersOverride {
    pub(crate) method: Option<Method>,
    pub(crate) scheme: Option<uri::Scheme>,
    pub(crate) authority: Option<BytesStr>,
    pub(crate) path: Option<BytesStr>,
    pub(crate) protocol: Option<Protocol>,
}

impl PseudoHeadersOverride {
    /// Creates an empty override set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the `:method` pseudo header.
    pub fn set_method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    /// Overrides the `:scheme` pseudo header.
    pub fn set_scheme(mut self, scheme: uri::Scheme) -> Self {
        self.scheme = Some(scheme);
        self
    }

    /// Overrides the `:authority` pseudo header using a parsed authority.
    pub fn set_authority(mut self, authority: uri::Authority) -> Self {
        self.authority = Some(BytesStr::from(authority.as_str()));
        self
    }

    /// Overrides the `:authority` pseudo header using a raw string value.
    pub fn set_authority_str(mut self, authority: &str) -> Self {
        self.authority = Some(BytesStr::from(authority));
        self
    }

    /// Overrides the `:path` pseudo header from a parsed path and query.
    pub fn set_path_and_query(mut self, path: uri::PathAndQuery) -> Self {
        self.path = Some(BytesStr::from(path.as_str()));
        self
    }

    /// Overrides the `:path` pseudo header using a raw string value.
    pub fn set_path(mut self, path: impl AsRef<str>) -> Self {
        self.path = Some(BytesStr::from(path.as_ref()));
        self
    }

    /// Overrides the `:protocol` pseudo header (RFC 8441 extended CONNECT).
    pub fn set_protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = Some(protocol);
        self
    }
}
