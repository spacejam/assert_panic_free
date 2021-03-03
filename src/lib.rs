#![no_std]

struct NoPanic;

extern "C" {
    #[link_name = "\n\nHEY! ACTUAL ERROR DOWN HERE!!! (sorry for that big mess up there) \
        the compiler was unable to prove that some code passed to assert_panic_free is \
        actually panic-free. Please ensure that optimizations are enabled."]
    fn trigger() -> !;
}

impl Drop for NoPanic {
    fn drop(&mut self) {
        unsafe {
            trigger();
        }
    }
}

/// A lightweight higher-order-function that doesn't compile
/// if a function you pass to it might panic. This probably
/// wont' work unless you're compiling your code with optimizations
/// enabled.
///
/// # Example
///
/// works when built with optimizations / release:
///
/// ```no_build
/// assert_panic_free::assert_panic_free(|| 32);
/// ```
///
/// doesn't work:
///
/// ```compile_fail
/// assert_panic_free(|| panic!(":("));
/// ```
pub fn assert_panic_free<R, F: FnOnce() -> R>(f: F) -> R {
    let guard = NoPanic;
    let result = f();
    core::mem::forget(guard);
    result
}
