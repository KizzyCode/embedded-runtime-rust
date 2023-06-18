//! Defines required runtime-specific function stubs

extern "Rust" {
    /// Blocks until an event occurs (may wake spuriously)
    ///
    /// # Important
    /// Events must not be lost. If an event has occurred between the last invocation and this invocation, this function
    /// must not block.
    pub(crate) fn _runtime_waitforevent_TBFzxdKN();
    /// Raises an event
    ///
    /// # Important
    /// Events must not be lost. If an event is sent, but the receiver is not currently waiting, it must be retained until
    /// the receiver tries to wait again.
    pub(crate) fn _runtime_sendevent_3YSaPmB7();
}
