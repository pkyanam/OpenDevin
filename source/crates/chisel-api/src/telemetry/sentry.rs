//! Sentry crash reporting.
//!
//! Reconstruction skeleton.

/// Initialise the Sentry SDK.
///
/// TODO(reconstruction): the original uses the `sentry` crate with the
/// `https://codeium-i5.sentry.io` DSN recovered from the binary strings.
pub fn init() {
    // TODO(reconstruction): sentry::init(DSN, ...) with panic + event capture.
}

pub fn capture_error(_e: &anyhow::Error) {
    // TODO(reconstruction): report the error to Sentry.
}