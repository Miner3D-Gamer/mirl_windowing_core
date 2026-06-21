/// Traits used by the backends
pub mod traits;

// #[cfg(feature = "console")]
// /// A console renderer
// pub mod console;

/// Errors that might occur when handling windows
pub mod errors;
pub use errors::*;

use mirl_buffer::traits::BufferMetrics;
use mirl_extensions::{TryTupleInto, TupleOps};
use mirl_input::mouse::{MouseButton, MouseButtonState};
use mirl_system::traits::WindowRenderLayer;

use crate::windowing::traits::MouseInput;
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(compactly = false))]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Settings for spawning in a window
pub struct WindowSettings<Os = mirl_system::Os> {
    /// Remove the border of the window
    pub borderless: bool,
    /// If the title should be displayed
    pub title_visible: bool,
    /// Window render level -> Topmost, normal, bottommost
    pub window_level: WindowRenderLayer,
    /// Position on screen
    pub position: (i32, i32),
    /// Size of the spawning window
    pub size: (i32, i32),
    /// If the window should be resizable
    pub resizable: bool,
    /// If the window should have the default os menu around it (fullscreen, minimize, close). The border however will still retain
    pub os_menu: bool,
    /// If the window should be considered visible to the user or not
    pub visible: bool,
    /// The os we're going for
    pub marker: core::marker::PhantomData<Os>,
}
use mirl_extensions::TryIntoPatch;

impl<T: mirl_system::traits::ObjectInCenterOfScreen + mirl_system::traits::Screen>
    WindowSettings<T>
{
    // TODO: Make this below a trait with variants both Self and Option<Self> are accepted as inputs
    /// Get the settings on default settings
    ///
    /// # Info
    /// For size a buffer is required
    /// For position, it will be centered on the screen
    ///
    /// # Errors
    /// If the buffer is too hight/wide to fit into a i32
    #[must_use]
    pub fn default<B: BufferMetrics>(buffer: &B) -> Option<Self> {
        let size = (buffer.width(), buffer.height()).try_into_value()?;
        Some(Self {
            borderless: false,
            title_visible: true,
            window_level: WindowRenderLayer::Normal,
            position: T::get_center_of_screen_for_object(size),

            resizable: false,
            os_menu: true,
            size,
            visible: true,
            marker: core::marker::PhantomData,
        })
    }
    /// Set the visibility of the window
    #[must_use]
    pub const fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }
    /// Set the size of the window
    ///
    /// If the given size is negative, the size will be set to (0, 0)
    #[must_use]
    pub const fn set_size(&mut self, size: (i32, i32)) -> &mut Self {
        self.size = (size.0.max(0), size.1.max(0));
        self
    }
    #[must_use]
    /// Set the size of the window to the size of the given buffer
    pub fn set_size_to_buffer<B: BufferMetrics>(mut self, buffer: &B) -> Option<Self> {
        self.size = (buffer.width(), buffer.height()).try_tuple_into()?;
        Some(self)
    }
    #[must_use]
    /// Set the position of the window
    pub const fn set_position(&mut self, position: (i32, i32)) -> &mut Self {
        self.position = position;
        self
    }
    #[must_use]
    /// Set if the title should be visible
    pub const fn set_title_visible(&mut self, title: bool) -> &mut Self {
        self.title_visible = title;
        self
    }
    #[must_use]
    /// Sets if the border should be hidden
    pub const fn set_borderless(&mut self, borderless: bool) -> &mut Self {
        self.borderless = borderless;
        self
    }
    #[must_use]
    /// Set the render level of the window (topmost, normal, bottommost)
    pub const fn set_window_level(&mut self, window_level: WindowRenderLayer) -> &mut Self {
        self.window_level = window_level;
        self
    }
    #[must_use]
    /// Set if the window should be resizable by the user
    ///
    /// TODO: Add more control to this. Most os allow for more fine tuning than just yes or no
    pub const fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }
    #[must_use]
    /// Set if the default os decoration should be visible (fullscreen, minimize, close)
    pub const fn set_os_menu(&mut self, os_menu: bool) -> &mut Self {
        self.os_menu = os_menu;
        self
    }
    #[must_use]
    #[cfg(any(target_arch = "wasm32", target_os = "linux", target_os = "windows"))]
    /// Sets the position of the window to be centered on the screen
    pub fn center_window(&mut self) -> &mut Self {
        self.position = T::get_center_of_screen_for_object(self.size);
        self
    }
    /// Multiply the size of the window
    #[must_use]
    pub const fn multiply_size(&mut self, by: i32) -> &mut Self {
        self.set_size(self.size.mul(by))
    }
    #[cfg(any(target_arch = "wasm32", target_os = "linux", target_os = "windows"))]
    /// Multiply the size of the window
    #[must_use]
    pub fn fullscreen(&mut self) -> Option<&mut Self> {
        // use crate::system::action::Screen;
        // The screen cannot be negative so unwrapping is safe
        self.size = T::get_screen_resolution().try_tuple_into()?;
        Some(self)
    }
}
pub const trait UpdateMouseButtonStateWithFramework {
    /// Update the current state by directly checking from the framework
    fn update_with_framework(&self, mouse: &mut MouseButtonState);
}
impl<T: MouseInput> UpdateMouseButtonStateWithFramework for T {
    /// Update the current state by directly checking from the framework
    fn update_with_framework(&self, mouse: &mut MouseButtonState) {
        mouse.update(
            self.is_mouse_down(MouseButton::Left),
            self.is_mouse_down(MouseButton::Middle),
            self.is_mouse_down(MouseButton::Right),
        );
    }
}
