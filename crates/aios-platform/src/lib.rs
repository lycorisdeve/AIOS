//! Native platform abstraction layer.
//!
//! This crate is where AIOS talks to real operating-system surfaces:
//! windows, input devices, display surfaces, notifications, files, and
//! application invocation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformKind {
    Windows,
    Macos,
    Linux,
    Android,
    Ios,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub scale_factor_milli: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAppTarget {
    pub app_id: String,
    pub action: String,
}

pub trait PlatformAdapter {
    fn platform_kind(&self) -> PlatformKind;
    fn primary_display(&self) -> DisplayInfo;
    fn invoke_app(&self, target: &NativeAppTarget) -> Result<(), PlatformError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformError {
    Unsupported,
    PermissionDenied,
    AppNotFound,
    InvocationFailed(String),
}

#[derive(Debug, Default)]
pub struct NullPlatformAdapter;

impl PlatformAdapter for NullPlatformAdapter {
    fn platform_kind(&self) -> PlatformKind {
        PlatformKind::Unknown
    }

    fn primary_display(&self) -> DisplayInfo {
        DisplayInfo {
            width: 1440,
            height: 900,
            scale_factor_milli: 1000,
        }
    }

    fn invoke_app(&self, _target: &NativeAppTarget) -> Result<(), PlatformError> {
        Err(PlatformError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_adapter_reports_unknown_platform() {
        let adapter = NullPlatformAdapter;

        assert_eq!(adapter.platform_kind(), PlatformKind::Unknown);
        assert_eq!(adapter.primary_display().width, 1440);
    }
}

