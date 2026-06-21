use mirl_buffer::Buffer;

use crate::windowing::{WindowError, WindowSettings, traits::WindowHelper};

/// A unified window creation method
pub const trait NewWindow {
    /// ### Create a new window with the desired settings
    ///
    /// ### Inputs:
    /// `title`: How the window should be named regardless of if it's shown
    ///
    /// `settings`: See [`WindowSettings`](crate::prelude::WindowSettings) for more info
    // /// `cursor`: If you wish to use cursors other than the default one, provide the cursor you want the window to show by default. If this is set to None, [`set_cursor_style()`](ExtendedWindow::set_cursor_style) may not work as intended
    /// # Errors
    /// See [`WindowError`] for the error messages
    fn new(
        title: &str,
        settings: WindowSettings,
        // #[cfg(feature = "svg")] cursor: Option<Cursor>,
    ) -> Result<Self, WindowError>
    where
        Self: Sized;
}

/// A window instance with only the most basic of functionality
///
/// Be aware that [`WindowHelper`] will automatically be implemented for all structs that implement this trait
pub const trait Window: WindowHelper {
    /// Update what the current window displays using a Buffer or \[u32]
    ///
    /// # Errors
    /// See [`WindowError`]
    fn update_raw(
        &mut self,
        pixels: &[u32],
        width: usize,
        height: usize,
    ) -> Result<(), WindowError>;
    /// Wether the current window is still open
    fn is_open(&self) -> bool;
    /// Clean up any remaining data after closing -> Otherwise memory leaks might happen
    fn close_and_clean_up(&mut self);
}
/// More 'advanced' window control
///
/// Like:
/// - Setting the title
pub const trait ExtendedWindow {
    /// Set the title (Duh)
    fn set_title(&mut self, title: &str);
}

/// More 'advanced' window control
pub const trait GetWindowHandle {
    /// Get the current window handle
    fn get_window_handle(&self) -> raw_window_handle::RawWindowHandle;
}
/// Set the icon of the window
pub const trait IconControl {
    /// Set the current icon (task bar)
    /// Width/Height should be something like 32x32 or 48x48 for maximal compatibility
    ///
    /// # Errors
    /// See [`WindowError`]
    fn set_icon(&mut self, buffer: &Buffer) -> Result<(), WindowError>;
}
// TODO: Add function to system that retrieves the current icon of a window
