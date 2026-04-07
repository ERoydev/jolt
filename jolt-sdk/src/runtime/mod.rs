mod boot;
mod exit;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub fn exit(code: i32) -> ! {
            std::process::exit(code)
        }
    } else if #[cfg(target_os = "none")] {
        pub use jolt_platform::putchar;

        pub fn exit(code: i32) -> ! {
            jolt_platform::platform_exit(code)
        }

        #[panic_handler]
        fn panic_handler(info: &core::panic::PanicInfo) -> ! {
            use core::fmt::Write;
            let _ = writeln!(jolt_platform::StderrWriter, "guest panicked: {info}");
            exit::__platform_abort(6) // SIGABRT
        }
    }
}

/// # Safety
/// `msg` must be null or a valid pointer to `len` bytes of readable memory.
#[no_mangle]
pub unsafe extern "C" fn __platform_stdout_write(msg: *const u8, len: usize) {
    if !msg.is_null() && len > 0 {
        let slice = core::slice::from_raw_parts(msg, len);
        for &byte in slice {
            jolt_platform::putchar(byte);
        }
    }
}
