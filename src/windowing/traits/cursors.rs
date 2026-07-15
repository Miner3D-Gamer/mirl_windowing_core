use mirl_input::mouse::{
    DefaultCursorColorInfo, LoadCursorError,
    cursors::{CursorResolution, DefaultCursors, RawCursor},
};
use mirl_system::cursors::UseCursorError;

// use crate::windowing::WindowError;

/// Load a cursor style into the native format
pub const trait ManageCursorStyle<Cursor> {
    /// Load the custom cursors Mirl provides by default
    ///
    /// # Errors
    /// [`LoadCursorError`]
    fn load_custom_default_cursors(
        &mut self,
        size: CursorResolution,
        color_info: DefaultCursorColorInfo,
    ) -> Result<DefaultCursors<Cursor>, LoadCursorError>;
    /// Load your own custom cursor
    /// Just make sure the size of the buffer is 32x32, 64x64, 128x128 or 256x256
    ///
    /// # Errors
    /// [`LoadCursorError`]
    fn load_custom_cursor(&mut self, cursor: RawCursor) -> Result<Cursor, LoadCursorError>;
    /// Set what cursors the os should display on the current window
    ///
    /// # Errors
    /// See [`WindowError`]
    fn set_cursor_style(&mut self, style: &Cursor) -> Result<(), UseCursorError>;
}

// /// Control over the cursor style while the mouse of hovering over it
// pub const trait UseCursorStyle {}
