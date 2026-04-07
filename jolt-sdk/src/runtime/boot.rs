use core::arch::naked_asm;

extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
}

#[cfg(target_os = "none")]
use buddy_system_allocator::LockedHeap;

#[cfg(target_os = "none")]
#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::new();

/// Hardware entry point. Sets gp and sp, then jumps to bootstrap.
/// # Safety
/// Must only be entered by firmware/boot code in a valid reset context.
#[cfg(target_os = "none")]
#[unsafe(naked)]
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        // Initialize global pointer first (RISC-V ABI requirement)
        ".weak __global_pointer$",
        ".hidden __global_pointer$",
        ".option push",
        ".option norelax",
        "   lla     gp, __global_pointer$",
        ".option pop",

        ".weak __stack_top",
        ".hidden __stack_top",
        "   lla     sp, __stack_top",
        "   andi    sp, sp, -16",

        "   tail    {bootstrap}",
        bootstrap = sym __bootstrap,
    )
}

/// # Safety
/// Must only be entered from `_start`.
#[cfg(target_os = "none")]
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn __bootstrap() -> ! {
    naked_asm!(
        "   call    {platform_bootstrap}",
        "   tail    {main}",
        platform_bootstrap = sym __platform_bootstrap,
        main = sym main,
    )
}

extern "Rust" {
    fn main() -> !;
}

#[no_mangle]
pub extern "C" fn __platform_bootstrap() {
    let heap_start = core::ptr::addr_of!(__heap_start) as usize;
    let heap_end = core::ptr::addr_of!(__heap_end) as usize;
    let heap_size = heap_end - heap_start;

    #[cfg(target_os = "none")]
    unsafe {
        HEAP.lock().init(heap_start, heap_size);
    }
}
