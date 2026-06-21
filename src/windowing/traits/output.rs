/// Basic logging
pub const trait Output {
    /// Log the given object (to the terminal)
    fn log(&self, t: &str);
}
