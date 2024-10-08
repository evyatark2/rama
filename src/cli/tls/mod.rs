//! CLI utilities for tls

#[cfg(feature = "rustls")]
pub mod rustls;

#[cfg(feature = "boring")]
pub mod boring;

#[cfg(feature = "boring")]
pub use boring::TlsServerCertKeyPair;

#[cfg(all(feature = "rustls", not(feature = "boring")))]
pub use rustls::TlsServerCertKeyPair;
