use std::ffi::c_char;

use super::{cairo::cairo_t, glib::{GError, GObject}};

#[repr(C)]
pub struct PopplerDocument(GObject);

#[repr(C)]
pub struct PopplerPage(GObject);

extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const c_char, 
        password: *const c_char, 
        error: *mut *mut GError) -> *mut PopplerDocument;

    pub fn poppler_document_get_page(document: *mut PopplerDocument, index: i32) -> *mut PopplerPage;

    pub fn poppler_document_get_n_pages(document: *mut PopplerDocument) -> i32;

    pub fn poppler_page_get_size(
        page: *mut PopplerPage, 
        width: *mut f64, height: *mut f64);

    pub fn poppler_page_render(page: *mut PopplerPage, cairo: *mut cairo_t);
}