#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::Array,
    programs::XdpContext,
};

#[map(name = "CYBERNETIC_MAP")]
static mut CYBERNETIC_MAP: Array<u32> = Array::with_max_entries(2, 0);

#[xdp]
pub fn soulvex_shield_xdp(ctx: XdpContext) -> u32 {
    match process_packets(ctx) {
        Ok(action) => action,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
fn process_packets(_ctx: XdpContext) -> Result<u32, u32> {
    unsafe {
        if let Some(block_mode) = CYBERNETIC_MAP.get(0) {
            if *block_mode == 1 {
                return Ok(xdp_action::XDP_DROP);
            }
        }
        
        if let Some(counter_ptr) = CYBERNETIC_MAP.get_ptr_mut(1) {
            if !counter_ptr.is_null() {
                *counter_ptr = (*counter_ptr).saturating_add(1);
            }
        }
    }
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}