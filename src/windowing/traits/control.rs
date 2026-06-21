use mirl_buffer::Buffer;
use mirl_system::traits::WindowRenderLayer;

/// Simple window management
pub const trait Control {
    /// Set the window position relative to its current position
    fn move_window(&mut self, xy: (i32, i32)) {
        let current = self.get_position();
        self.set_position((current.0 + xy.0, current.1 + xy.1));
    }
    /// Get the position of the window
    fn get_position(&self) -> (i32, i32);
    /// Set the position of the window
    fn set_position(&mut self, xy: (i32, i32));
    /// Set the size of the window using the dimensions of a Buffer
    fn set_size(&mut self, buffer: &Buffer);
    /// Get the size of the window
    fn get_size(&self) -> (i32, i32);
}

/// More complex window controls
pub const trait Visibility {
    /// Minimize the window
    fn minimize(&mut self);
    /// Maximize the window (More or less fullscreen), to un-minimize use [`minimize()`](Visibility::minimize)
    fn maximize(&mut self);
    /// Opposite of [`minimize()`](Visibility::minimize)
    fn restore(&mut self);
    /// Wether the current window is minimized
    fn is_minimized(&self) -> bool;
    /// Wether the current window is maximized
    fn is_maximized(&self) -> bool;
}
/// Control the render layer of the given window
pub const trait RenderLayer {
    /// Set the window layer like topmost, bottommost, and default
    fn set_render_layer(&mut self, render_layer: WindowRenderLayer);
}
