use std::{ffi::c_char, os::raw::c_void};

#[repr(C)]
pub struct GError {
    pub domain: u32,       // GLib defines domain as guint32
    pub code: i32,         // Error code, typically gint
    pub message: *mut c_char, // Pointer to the error message (C string)
}

#[repr(C)]
pub struct GObject(c_void);

extern "C" {
    pub fn g_object_unref(object: *mut GObject);
    pub fn g_error_free(error: *mut GError);
}