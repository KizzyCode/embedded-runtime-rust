//! Provides the runtime specific functions for an rp2040 platform

use cortex_m::asm;

/// Blocks until an event occurs (may wake spuriously)
///
/// # Important
/// Events must not be lost. If an event has occurred between the last invocation and this invocation, this function must
/// not block.
#[no_mangle]
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn _runtime_waitforevent_TBFzxdKN() {
    // Wait until an event occurrs
    // Deadlock considerations: Events are retained, so if an interrupt happens or an event is sent before the next
    // invocation of `wfe`, it will not block.
    //
    // See also: https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/ARMv6-M-exception-model/Wait-For-Event-and-Send-Event?lang=en
    asm::wfe()
}

/// Raises an event
///
/// # Important
/// Events must not be lost. If an event is sent, but the receiver is not currently waiting, it must be retained until the
/// receiver tries to wait again.
#[no_mangle]
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn _runtime_sendevent_3YSaPmB7() {
    // Sends an event
    // Deadlock considerations: Events are retained, so if an interrupt happens or an event is sent before the next
    // invocation of `wfe`, it will not block.
    //
    // See also: https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/ARMv6-M-exception-model/Wait-For-Event-and-Send-Event?lang=en
    asm::sev()
}
