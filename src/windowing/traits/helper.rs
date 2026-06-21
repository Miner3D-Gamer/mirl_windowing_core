use mirl_buffer::Buffer;
use mirl_extensions::*;
use mirl_input::mouse::{MouseButton, MouseSnapShot};

use super::*;
use crate::windowing::WindowError;

/// Most basic of framework functionality
impl<
    T: Window
        + MouseInput
        + KeyboardInput
        + Output
        // + Timing
        + WindowHelper
        + core::fmt::Debug,
> WindowingFramework for T
{
}
/// Most basic of framework functionality
pub const trait WindowingFramework:
    Window + MouseInput + Output + WindowHelper + core::fmt::Debug
{
}
// #[cfg(all(feature = "svg"))]
impl<
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + ManageCursorStyle<Cursor>
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + KeyboardInput
        + ExtendedKeyboardInput
        + GetWindowHandle,
    Cursor,
> ExtendedWindowingFramework<Cursor> for T
{
}

// // system + keycodes, no svg
// #[cfg(all(not(feature = "svg")))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework<Cursor>:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + KeyboardInput
    + ExtendedKeyboardInput
    + GetWindowHandle
    + ManageCursorStyle<Cursor>
{
}
/// A simple helper that implements functions that mustn't be written by hand
pub const trait WindowHelper {
    /// Update what the current window displays using a Buffer
    ///
    /// # Errors
    /// See [`WindowError`]
    fn update(&mut self, buffer: &Buffer) -> Result<(), WindowError>;
}
impl<T: Window> WindowHelper for T {
    fn update(&mut self, buffer: &Buffer) -> Result<(), WindowError> {
        self.update_raw(buffer, buffer.width, buffer.height)
    }
}

/// A simple helper that implements input related functions that mustn't be written by hand
// #[cfg(feature = "std")]
pub const trait WindowInputHelper {
    /// Get a snapshot of the current state of the mouse
    fn get_mouse_snapshot(&self) -> MouseSnapShot;
}
// #[cfg(feature = "std")]
impl<T: MouseInput + ExtendedMouseInput> WindowInputHelper for T {
    fn get_mouse_snapshot(&self) -> MouseSnapShot {
        MouseSnapShot {
            position: self.get_mouse_position(),
            scroll: self.get_mouse_scroll(),
            left_down: self.is_mouse_down(MouseButton::Left),
            middle_down: self.is_mouse_down(MouseButton::Middle),
            right_down: self.is_mouse_down(MouseButton::Right),
        }
    }
}

// /// A helper that is
// pub const trait DeprecatedCompatibilityHelper {
//     /// This function is deprecated as it is annoying to type out
//     ///
//     /// It is recommended to use [`Window::update_raw`] or [`WindowHelper::update`] instead
//     /// # Errors
//     /// See [`WindowError`]
//     #[deprecated]
//     fn update_with_buffer(
//         &mut self,
//         buffer: &Buffer,
//     ) -> Result<(), WindowError>;
// }

// impl<T: Window + Control> DeprecatedCompatibilityHelper for T {
//     fn update_with_buffer(
//         &mut self,
//         buffer: &Buffer,
//     ) -> Result<(), WindowError> {
//         self.update(buffer)
//     }
// }
/// Get the relative mouse position
pub const trait RelativeMousePos {
    /// Get the mouse position relative to the window
    fn get_mouse_position_relative(&self) -> Option<(f32, f32)>;
}

impl<T: MouseInput + Control> RelativeMousePos for T {
    fn get_mouse_position_relative(&self) -> Option<(f32, f32)> {
        let mouse_pos = self.get_mouse_position()?;
        let window_pos = self.get_position().try_tuple_into()?;
        Some(mouse_pos.sub(window_pos))
    }
}
