use std::{
    ffi::c_char,
    os::raw::c_void,
};
use std::ptr::copy_nonoverlapping;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CairoFormat {
    Invalid = -1,
    Argb32 = 0,
    Rgb24 = 1,
    A8 = 2,
    A1 = 3,
    Rgb16_565 = 4,
    Rgb30 = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum cairo_status_t {
    CAIRO_STATUS_SUCCESS = 0,

    CAIRO_STATUS_NO_MEMORY,
    CAIRO_STATUS_INVALID_RESTORE,
    CAIRO_STATUS_INVALID_POP_GROUP,
    CAIRO_STATUS_NO_CURRENT_POINT,
    CAIRO_STATUS_INVALID_MATRIX,
    CAIRO_STATUS_INVALID_STATUS,
    CAIRO_STATUS_NULL_POINTER,
    CAIRO_STATUS_INVALID_STRING,
    CAIRO_STATUS_INVALID_PATH_DATA,
    CAIRO_STATUS_READ_ERROR,
    CAIRO_STATUS_WRITE_ERROR,
    CAIRO_STATUS_SURFACE_FINISHED,
    CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
    CAIRO_STATUS_PATTERN_TYPE_MISMATCH,
    CAIRO_STATUS_INVALID_CONTENT,
    CAIRO_STATUS_INVALID_FORMAT,
    CAIRO_STATUS_INVALID_VISUAL,
    CAIRO_STATUS_FILE_NOT_FOUND,
    CAIRO_STATUS_INVALID_DASH,
    CAIRO_STATUS_INVALID_DSC_COMMENT,
    CAIRO_STATUS_INVALID_INDEX,
    CAIRO_STATUS_CLIP_NOT_REPRESENTABLE,
    CAIRO_STATUS_TEMP_FILE_ERROR,
    CAIRO_STATUS_INVALID_STRIDE,
    CAIRO_STATUS_FONT_TYPE_MISMATCH,
    CAIRO_STATUS_USER_FONT_IMMUTABLE,
    CAIRO_STATUS_USER_FONT_ERROR,
    CAIRO_STATUS_NEGATIVE_COUNT,
    CAIRO_STATUS_INVALID_CLUSTERS,
    CAIRO_STATUS_INVALID_SLANT,
    CAIRO_STATUS_INVALID_WEIGHT,
    CAIRO_STATUS_INVALID_SIZE,
    CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED,
    CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
    CAIRO_STATUS_DEVICE_ERROR,
    CAIRO_STATUS_INVALID_MESH_CONSTRUCTION,
    CAIRO_STATUS_DEVICE_FINISHED,
    CAIRO_STATUS_JBIG2_GLOBAL_MISSING,
    CAIRO_STATUS_PNG_ERROR,
    CAIRO_STATUS_FREETYPE_ERROR,
    CAIRO_STATUS_WIN32_GDI_ERROR,
    CAIRO_STATUS_TAG_ERROR,

    CAIRO_STATUS_LAST_STATUS,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cairo_surface_t(c_void);

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cairo_t(c_void);

pub struct ByteBuffer {
    pub data: *mut u8,
    pub size: usize,
    pub capacity: usize,
}

pub extern "C" fn write_to_memory(closure: *mut c_void, data: *const u8, length: u32) -> cairo_status_t {
    unsafe {
        let buffer = &mut *(closure as *mut ByteBuffer);

        if buffer.data.is_null() {
            buffer.data = libc::malloc(length as usize) as *mut u8;
            if buffer.data.is_null() {
                return cairo_status_t::CAIRO_STATUS_NO_MEMORY;
            }
            buffer.capacity = length as usize;
        } else if buffer.size + length as usize > buffer.capacity {
            let new_capacity = buffer.size + length as usize;
            let new_data = libc::realloc(buffer.data as *mut c_void, new_capacity) as *mut u8;
            if new_data.is_null() {
                return cairo_status_t::CAIRO_STATUS_NO_MEMORY;
            }
            buffer.data = new_data;
            buffer.capacity = new_capacity;
        }

        copy_nonoverlapping(data, buffer.data.add(buffer.size), length as usize);
        buffer.size += length as usize;

        cairo_status_t::CAIRO_STATUS_SUCCESS
    }
}


type CairoWriteFunc = extern "C" fn(*mut c_void, *const u8, u32) -> cairo_status_t;


extern "C" {
    pub fn cairo_image_surface_create(
        format: CairoFormat,
        width: i32,
        height: i32,
    ) -> *mut cairo_surface_t;

    pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_status(cairo: *mut cairo_t) -> cairo_status_t;

    pub fn cairo_status_to_string(status: cairo_status_t) -> *const c_char;

    pub fn cairo_create(surface: *mut cairo_surface_t) -> *mut cairo_t;

    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_destroy(cairo: *mut cairo_t);

    pub fn cairo_scale(cairo: *mut cairo_t, sx: f64, sy: f64);

    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);

    pub fn cairo_surface_write_to_png(
        surface: *mut cairo_surface_t,
        filename: *const c_char,
    ) -> cairo_status_t;

    pub fn cairo_surface_write_to_png_stream(
        surface: *mut cairo_surface_t,
        write_func: CairoWriteFunc, 
        closure: *mut c_void) -> cairo_status_t;
}
