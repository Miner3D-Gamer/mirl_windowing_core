use mirl_core::platform::KeyCode;
use mirl_input::mouse::MouseButton;

/// Basic input detection
pub const trait MouseInput {
    /// Gets the current mouse position
    fn get_mouse_position(&self) -> Option<(f32, f32)>;
    /// Checks if the requested mouse button is down
    fn is_mouse_down(&self, button: MouseButton) -> bool;
}

/// More advanced input methods
pub const trait ExtendedMouseInput {
    /// Get how much the mouse has been scrolled on its wheel (x, y)
    fn get_mouse_scroll(&self) -> (f32, f32);
}

/// Basic input detection
pub const trait KeyboardInput {
    /// Checks if the requested key is down.
    /// Warning: Most backends to not support all keys (like 'f25', 'world2', or 'Þ') and will always return false in that case
    fn is_key_down(&self, key: KeyCode) -> bool;
}

/// More advanced input methods
pub const trait ExtendedKeyboardInput {
    /// Get all currently pressed keys
    fn get_all_keys_down(&self) -> Vec<KeyCode>;
}
