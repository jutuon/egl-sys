
#[cfg(unix)]
extern crate x11;

#[cfg_attr(not(feature = "function_pointer_loading"), link(name="EGL"))]
extern {}

pub(crate) mod platform_types {
    #![allow(non_camel_case_types)]

    // From khrplatform.h and eglplatform.h

    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = isize;
    pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    pub type EGLint = i32;

    pub use platform_types::display_pixmap_window::*;

    pub type NativeDisplayType = EGLNativeDisplayType;
    pub type NativePixmapType = EGLNativePixmapType;
    pub type NativeWindowType = EGLNativeWindowType;


    #[cfg(unix)]
    pub mod display_pixmap_window {
        use x11;

        pub type EGLNativeDisplayType = *mut x11::xlib::Display;
        pub type EGLNativePixmapType = x11::xlib::Pixmap;
        pub type EGLNativeWindowType = x11::xlib::Window;
    }

}


pub mod ffi {
    #[cfg(not(feature = "racer_autocomplete_workaround"))]
    include!(concat!(env!("OUT_DIR"), "/egl_generated_bindings.rs"));

    #[cfg(feature = "racer_autocomplete_workaround")]
    pub use egl_generated_bindings::*;
}


#[cfg(feature = "extensions_module")]
pub mod extensions {
    #[cfg(not(feature = "racer_autocomplete_workaround"))]
    include!(concat!(env!("OUT_DIR"), "/egl_generated_extension_bindings.rs"));

    #[cfg(feature = "racer_autocomplete_workaround")]
    pub use egl_generated_extension_bindings::*;
}


#[cfg(feature = "racer_autocomplete_workaround")]
mod egl_generated_bindings;

#[cfg(all(feature = "racer_autocomplete_workaround", feature = "extensions_module"))]
mod egl_generated_extension_bindings;