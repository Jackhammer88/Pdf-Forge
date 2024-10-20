use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use document::Document;
use page::Page;
use ffi::cairo::ByteBuffer;

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
        None => -1, // Возвращаем -1 в случае ошибки
    }
}

// Получение страницы
#[no_mangle]
pub extern "C" fn document_get_page(doc: *mut DocumentWrapper, page_number: c_int, scale: f64) -> *mut PageWrapper {
    let document = unsafe { &mut (*doc).document };
    match document {
        Some(d) => match d.get_page(page_number, scale) {
            Ok(page) => Box::into_raw(Box::new(PageWrapper { page: Some(page) })),
            Err(_) => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

// Сохранение страницы как PNG
#[no_mangle]
pub extern "C" fn page_save_as_png(page: *mut PageWrapper, filename: *const c_char) -> c_int {
    // Используем CStr вместо CString, чтобы не брать управление памятью
    let c_str = unsafe {
        if filename.is_null() {
            return -1;
        }
        CStr::from_ptr(filename)
    };
    
    let page_ref = unsafe { &mut (*page).page };

    match page_ref {
        Some(p) => {
            if p.to_png_file(&c_str.to_string_lossy()).is_ok() {
                0 // Успех
            } else {
                -1 // Ошибка
            }
        },
        None => -1, // Ошибка, если страница не найдена
    }
}

// Обработка FFI
#[no_mangle]
pub extern "C" fn page_to_png_bytes(page: *mut PageWrapper) -> *mut ByteBuffer {
    let page_ref = unsafe { &mut (*page).page };

    match page_ref {
        Some(p) => {
            // Преобразуем байты в Box<[u8]>
            let boxed_bytes = p.to_memory_box();
            let buffer = ByteBuffer {
                data: boxed_bytes.as_ptr() as *mut u8,
                size: boxed_bytes.len(),
                capacity: boxed_bytes.len(),
            };

            // Нам нужно предотвратить освобождение памяти, используя Box::leak для утечки данных
            std::mem::forget(boxed_bytes); // Этот вызов гарантирует, что память не будет освобождена Rust-ом
            Box::into_raw(Box::new(buffer)) // Передаем буфер в C#
        }
        None => std::ptr::null_mut(),
    }
}

// Функция для освобождения ресурсов документа
#[no_mangle]
pub extern "C" fn document_free(doc: *mut DocumentWrapper) {
    if !doc.is_null() {
        unsafe {
            Box::from_raw(doc); // Автоматически освободим память
        }
    }
}

// Функция для освобождения ресурсов страницы
#[no_mangle]
pub extern "C" fn page_free(page: *mut PageWrapper) {
    if !page.is_null() {
        unsafe {
            Box::from_raw(page); // Автоматически освободим память
        }
    }
}

// Функция для получения данных буфера
#[no_mangle]
pub extern "C" fn get_byte_buffer_data(buffer: *mut ByteBuffer) -> *const u8 {
    unsafe { (*buffer).data }
}

// Функция для получения размера буфера
#[no_mangle]
pub extern "C" fn get_byte_buffer_size(buffer: *mut ByteBuffer) -> usize {
    unsafe { (*buffer).size }
}

// Получение емкости буфера
#[no_mangle]
pub extern "C" fn get_byte_buffer_capacity(buffer: *mut ByteBuffer) -> usize {
    unsafe { (*buffer).capacity }
}

#[no_mangle]
pub extern "C" fn free_byte_buffer(buffer: *mut ByteBuffer) {
    if !buffer.is_null() {
        unsafe {
            // Освобождаем данные внутри буфера
            libc::free((*buffer).data as *mut c_void);

            // Освобождаем саму структуру буфера
            Box::from_raw(buffer); 
        }
    }
}
