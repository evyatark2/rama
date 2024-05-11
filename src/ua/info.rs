use std::{fmt, str::FromStr};

use super::{parse_http_user_agent, UserAgentParseError};

/// User Agent (UA) information.
///
/// See [the module level documentation](crate::ua) for more information.
#[derive(Debug, Clone)]
pub struct UserAgent {
    pub(super) data: UserAgentData,
}

/// internal representation of the [`UserAgent`]
#[derive(Debug, Clone)]
pub(super) enum UserAgentData {
    Known(UserAgentInfo),
    Desktop,
    Mobile,
}

/// Information about the [`UserAgent`]
#[derive(Debug, Clone)]
pub(super) struct UserAgentInfo {
    /// The 'User-Agent' http header value used by the [`UserAgent`].
    pub(super) http_user_agent: String,

    /// The kind of [`UserAgent`]
    pub(super) kind: Option<UserAgentKind>,
    /// The major version of the [`UserAgent`]
    pub(super) version: Option<usize>,

    /// The PlatformKind used by the [`UserAgent`]
    pub(super) platform: Option<PlatformKind>,
}

impl FromStr for UserAgent {
    type Err = UserAgentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_http_user_agent(s)
    }
}

impl UserAgent {
    /// returns the 'User-Agent' http header value used by the [`UserAgent`].
    pub fn header_str(&self) -> Option<&str> {
        if let UserAgentData::Known(info) = &self.data {
            Some(&info.http_user_agent)
        } else {
            None
        }
    }

    /// returns the device kind of the [`UserAgent`].
    pub fn device(&self) -> DeviceKind {
        match &self.data {
            UserAgentData::Known(info) => match info.platform {
                Some(PlatformKind::Windows | PlatformKind::MacOS | PlatformKind::Linux) | None => {
                    DeviceKind::Desktop
                }
                Some(PlatformKind::Android | PlatformKind::IOS) => DeviceKind::Mobile,
            },
            UserAgentData::Desktop => DeviceKind::Desktop,
            UserAgentData::Mobile => DeviceKind::Mobile,
        }
    }

    /// returns the kind of [`UserAgent`], if known.
    pub fn kind(&self) -> Option<UserAgentKind> {
        if let UserAgentData::Known(info) = &self.data {
            info.kind
        } else {
            None
        }
    }

    /// returns the major version of the [`UserAgent`], if known.
    ///
    /// This is the version of the distribution, not the version a component such as the rendering engine.
    pub fn version(&self) -> Option<usize> {
        if let UserAgentData::Known(info) = &self.data {
            info.version
        } else {
            None
        }
    }

    /// returns the [`PlatformKind`] used by the [`UserAgent`], if known.
    ///
    /// This is the platform the UA is running on.
    pub fn platform(&self) -> Option<PlatformKind> {
        if let UserAgentData::Known(info) = &self.data {
            info.platform
        } else {
            None
        }
    }

    /// returns the [`HttpAgent`] used by the [`UserAgent`].
    ///
    /// [`UserAgent`]: crate::ua::UserAgent
    pub fn http_agent(&self) -> HttpAgent {
        self.kind()
            .map(|kind| match kind {
                UserAgentKind::Chromium => HttpAgent::Chromium,
                UserAgentKind::Firefox => HttpAgent::Firefox,
                UserAgentKind::Safari => HttpAgent::Safari,
            })
            .unwrap_or(HttpAgent::Chromium)
    }

    /// returns the [`TlsAgent`] used by the [`UserAgent`].
    ///
    /// [`UserAgent`]: crate::ua::UserAgent
    pub fn tls_agent(&self) -> TlsAgent {
        self.kind()
            .map(|kind| match kind {
                UserAgentKind::Chromium => TlsAgent::Boringssl,
                UserAgentKind::Firefox | UserAgentKind::Safari => TlsAgent::Rustls,
            })
            .unwrap_or(TlsAgent::Rustls)
    }
}

/// The kind of [`UserAgent`]
///
/// [`UserAgent`]: crate::ua::UserAgent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserAgentKind {
    /// Chromium Browser
    Chromium,
    /// Firefox Browser
    Firefox,
    /// Safari Browser
    Safari,
}

impl fmt::Display for UserAgentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAgentKind::Chromium => write!(f, "Chromium"),
            UserAgentKind::Firefox => write!(f, "Firefox"),
            UserAgentKind::Safari => write!(f, "Safari"),
        }
    }
}

/// Device on which the [`UserAgent`] operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    /// Personal Computers
    Desktop,
    /// Phones, Tablets and other mobile devices
    Mobile,
}

impl fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceKind::Desktop => write!(f, "Desktop"),
            DeviceKind::Mobile => write!(f, "Mobile"),
        }
    }
}

/// Platform within the [`UserAgent`] operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformKind {
    /// Windows Platform (desktop)
    Windows,
    /// MacOS Platform (desktop)
    MacOS,
    /// Linux Platform (desktop)
    Linux,
    /// Android Platform (mobile)
    Android,
    /// iOS Platform (mobile)
    IOS,
}

impl fmt::Display for PlatformKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformKind::Windows => write!(f, "Windows"),
            PlatformKind::MacOS => write!(f, "MacOS"),
            PlatformKind::Linux => write!(f, "Linux"),
            PlatformKind::Android => write!(f, "Android"),
            PlatformKind::IOS => write!(f, "iOS"),
        }
    }
}

/// Http implementation used by the [`UserAgent`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpAgent {
    /// Chromium based browsers share the same http implementation
    Chromium,
    /// Firefox has its own http implementation
    Firefox,
    /// Safari also has its own http implementation
    Safari,
}

impl fmt::Display for HttpAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpAgent::Chromium => write!(f, "Chromium"),
            HttpAgent::Firefox => write!(f, "Firefox"),
            HttpAgent::Safari => write!(f, "Safari"),
        }
    }
}

/// Tls implementation used by the [`UserAgent`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TlsAgent {
    /// Rustls is used as a fallback for all user agents,
    /// that are not chromium based.
    Rustls,
    /// Boringssl is used for Chromium based user agents.
    Boringssl,
    /// NSS is used for Firefox
    Nss,
}

impl fmt::Display for TlsAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsAgent::Rustls => write!(f, "Rustls"),
            TlsAgent::Boringssl => write!(f, "Boringssl"),
            TlsAgent::Nss => write!(f, "NSS"),
        }
    }
}
