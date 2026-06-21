#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Potential errors that may occur when trying to open a window
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
pub enum WindowCreationError {
    /// The given backend could not load itself before even opening the window
    BackendFailedToLoad,
    /// The transparency feature requires the borderless property when on windows
    TransparencyRequiresBorderlessProperty,
    /// The os was unable to create a window - Why is unknown
    OsFailed,
    /// An unaccounted error occurrence
    Misc(String),
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Potential errors that may occur when trying to update a window
pub enum WindowUpdateError {
    /// When the given buffer and specified size have incorrect sizes
    BufferInvalidSize {
        /// Width of the expected buffer
        width: usize,
        /// Idk either ¯\_(ツ)_/¯
        stride: usize,
        /// Height of the expected buffer
        height: usize,
        /// Expected size of the buffer
        expected: usize,
        /// Gotten size of the buffer
        gotten: usize,
    },

    /// An unaccounted error occurrence
    Misc(String),
}
impl From<WindowUpdateError> for WindowError {
    fn from(value: WindowUpdateError) -> Self {
        Self::FailedToUpdateWindow(value)
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// An enum for checking what kind of error was produced
pub enum WindowError {
    /// When for example the buffer was too small for the window, contains the expected buffer size
    IncorrectSize((usize, usize)),
    /// Creating a window is not supported on this os - maybe try another Framework
    OsNotSupported,
    /// Failed to create window -> Creating a window on this os may be supported but something else went wrong
    FailedToOpenWindow(WindowCreationError),
    /// Failed to update window
    FailedToUpdateWindow(WindowUpdateError),
    /// When a feature is not yet implemented
    NotImplemented,
    /// When a window already exists
    DuplicateWindow,
    /// When accessing a file wasn't possible
    FileAccessNotPossible {
        /// The file path that was accessed
        path: String,
    },
    /// When unable to load an application icon (taskbar)
    UnableToLoadIcon,
    /// When unable to load a cursor
    UnableToLoadCursor,
    /// Other error
    Misc(String),
}
impl std::fmt::Display for WindowCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendFailedToLoad => {
                write!(f, "Backend failed to load before opening the window")
            }
            Self::TransparencyRequiresBorderlessProperty => {
                write!(
                    f,
                    "Transparency feature requires the borderless property on Windows"
                )
            }
            Self::OsFailed => {
                write!(f, "Operating system failed to create a window")
            }
            Self::Misc(msg) => {
                write!(f, "Window creation error: {msg}")
            }
        }
    }
}

impl std::error::Error for WindowCreationError {}

impl std::fmt::Display for WindowUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferInvalidSize {
                width,
                stride,
                height,
                expected,
                gotten,
            } => {
                write!(
                    f,
                    "Buffer has invalid size: expected {expected} bytes for {width}x{height} (stride: {stride}), got {gotten} bytes"
                )
            }
            Self::Misc(msg) => {
                write!(f, "Window update error: {msg}")
            }
        }
    }
}

impl std::error::Error for WindowUpdateError {}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectSize((width, height)) => {
                write!(f, "Incorrect buffer size: expected {width}x{height}")
            }
            Self::OsNotSupported => {
                write!(
                    f,
                    "Window creation is not supported on this operating system"
                )
            }
            Self::FailedToOpenWindow(err) => {
                write!(f, "Failed to open window: {err}")
            }
            Self::FailedToUpdateWindow(err) => {
                write!(f, "Failed to update window: {err}")
            }
            Self::NotImplemented => {
                write!(f, "Feature is not yet implemented")
            }
            Self::DuplicateWindow => {
                write!(f, "A window already exists")
            }
            Self::FileAccessNotPossible {
                path,
            } => {
                write!(f, "Unable to access file: {path}")
            }
            Self::UnableToLoadIcon => {
                write!(f, "Unable to load application icon")
            }
            Self::UnableToLoadCursor => {
                write!(f, "Unable to load cursor")
            }
            Self::Misc(msg) => {
                write!(f, "Window error: {msg}")
            }
        }
    }
}

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FailedToOpenWindow(err) => Some(err),
            Self::FailedToUpdateWindow(err) => Some(err),
            _ => None,
        }
    }
}
