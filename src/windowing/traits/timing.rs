// /// Basic timing control
// pub const trait Timing {
//     /// Get the current time
//     ///
//     /// TODO: A box isn't needed for this
//     fn get_time(&self) -> std::time::Instant;
//     /// Sleep for X milliseconds - You don't need more accuracy, do you?
//     fn sleep(&self, time: std::time::Duration);
//     // /// Get
//     // fn get_delta_time(&mut self) -> f64;
// }

/// Advanced timing control
pub const trait ExtendedTiming {
    /// Automatically set the max fps, use manual fps management for more control
    fn set_target_fps(&mut self, fps: usize);
}
