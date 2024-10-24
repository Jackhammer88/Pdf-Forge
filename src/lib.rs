use std::{ffi::{c_char, CStr}, ptr};

use document::Document;
use libc::c_int;
use page::Page;

mod document;
mod page;
pub mod ffi;

#[repr(C)]
pub struct DocumentWrapper {
    document: Option<Document>, // Оборачиваем наш объект Document
}

#[repr(C)]
pub struct PageWrapper {
    page: Option<Page>, // Оборачиваем наш объект Page
}

#[repr(C)]
pub struct ByteBuffer {
    data: *mut u8,
    size: usize
}

// Функция для создания документа
#[no_mangle]
pub extern "C" fn document_new(filename: *const c_char) -> *mut DocumentWrapper {
    // Используем CStr вместо CString, чтобы не брать управление памятью
    let c_str = unsafe {
        if filename.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(filename)
    };

    match Document::new(c_str.to_str().unwrap()) {
        Ok(doc) => Box::into_raw(Box::new(DocumentWrapper { document: Some(doc) })),
        Err(_) => ptr::null_mut(),
    }
}

// Получение общего количества страниц
#[no_mangle]
pub extern "C" fn document_total_pages(doc: *mut DocumentWrapper) -> c_int {
    let document = unsafe { &(*doc).document };
    match document {
        Some(d) => d.total_pages() as c_int,
        None => -1,
    }
}

// Получение страницы
#[no_mangle]
pub extern "C" fn document_get_page(doc: *mut DocumentWrapper, page_number: c_int) -> *mut PageWrapper {
    let document = unsafe { &mut (*doc).document };
    match document {
        Some(d) => match d.get_page(page_number) {
            Ok(page) => Box::into_raw(Box::new(PageWrapper { page: Some(page) })),
            Err(_) => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn page_to_png_bytes(page: *mut PageWrapper, scale: f64) -> *mut ByteBuffer {
    let page_ref = unsafe { &mut (*page).page };

    match page_ref {
        Some(p) => {
            match p.to_png_bytes(scale) {
                Ok(bytes) => {
                    // Convert Vec<u8> into Box<[u8]> to transfer ownership
                    let data_box = bytes.into_boxed_slice();
                    let size = data_box.len();
                    let data_ptr = Box::into_raw(data_box) as *mut u8;

                    let buffer = ByteBuffer {
                        data: data_ptr,
                        size,
                    };

                    // Transfer ownership of the ByteBuffer to the caller
                    Box::into_raw(Box::new(buffer))
                },
                Err(_) => std::ptr::null_mut(),
            }
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn page_save_as_png(page: *mut PageWrapper, filename: *const c_char, scale: f64) -> c_int {
    let c_filename = unsafe {
        if filename.is_null() {
            return -1;
        }
        CStr::from_ptr(filename)
    };

    let page_ref = unsafe { &mut (*page).page };

    match page_ref {
        Some(p) => {
            if p.to_png_file(&c_filename.to_string_lossy(), scale).is_ok() {
                0
            } else {
                -1
            }
        },
        None => -1,
    }
}

// Функция для освобождения ресурсов документа
#[no_mangle]
pub extern "C" fn document_free(doc: *mut DocumentWrapper) {
    if !doc.is_null() {
        unsafe {
            let _ = Box::from_raw(doc);
        }
    }
}

// Функция для освобождения ресурсов страницы
#[no_mangle]
pub extern "C" fn page_free(page: *mut PageWrapper) {
    if !page.is_null() {
        unsafe {
            let _ = Box::from_raw(page);
        }
    }
}

// Функция для освобождения данных буфера
#[no_mangle]
pub extern "C" fn free_byte_buffer(buffer: *mut ByteBuffer) {
    if !buffer.is_null() {
        unsafe {
            let data_ptr = (*buffer).data;
            let size = (*buffer).size;
            if !data_ptr.is_null() && size > 0 {
                let data_slice = std::slice::from_raw_parts_mut(data_ptr, size);
                let _data_box = Box::from_raw(data_slice as *mut [u8]);
            }

            // Reconstruct the ByteBuffer to free it
            let _ = Box::from_raw(buffer);
        }
    }
}
