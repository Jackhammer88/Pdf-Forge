use anyhow::{self, bail, Context, Ok, Result};
use std::ffi::CString;

use crate::ffi::glib::{g_error_free, g_object_unref, GError, GObject};
use crate::ffi::poppler::{
    poppler_document_get_n_pages, poppler_document_get_page, poppler_document_new_from_file,
    PopplerDocument,
};
use crate::page::Page;

pub struct Document {
    filename: String,
    total_pages: i32,
    poppler_document: *mut PopplerDocument,
}

impl Document {
    pub fn new(filename: &str) -> Result<Document> {
        let poppler_document = unsafe {
            let mut g_error: *mut GError = std::ptr::null_mut();

            let c_filename =
                CString::new(filename).context("Can't get correct file name to pdf document.")?;
            let document =
                poppler_document_new_from_file(c_filename.as_ptr(), std::ptr::null(), &mut g_error)
                    .as_mut();

            if !g_error.is_null() {
                // Получаем сообщение об ошибке из GError
                let error_message = std::ffi::CStr::from_ptr((*g_error).message)
                    .to_string_lossy()
                    .into_owned();
                g_error_free(g_error); // Освобождаем ошибку

                bail!(format!("Poppler error: {}", error_message));
            }

            if document.is_none() {
                bail!("Failed to create Poppler document.");
            }

            Ok(document.unwrap())
        }?;

        // open document and get total page count
        let total_pages = unsafe { poppler_document_get_n_pages(poppler_document) };

        Ok(Document {
            filename: filename.to_string(),
            total_pages,
            poppler_document,
        })
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn total_pages(&self) -> i32 {
        self.total_pages
    }

    pub fn get_page(&mut self, number: i32, scale: f64) -> Result<Page> {
        self.get_poppler_page(number, scale)
    }

    fn get_poppler_page(&self, number: i32, scale: f64) -> Result<Page> {
        let poppler_page =
            unsafe { poppler_document_get_page(self.poppler_document, number).as_mut() };

        match poppler_page {
            Some(p) => Page::new(p, scale),
            None => bail!(format!("Page {} not found in the document.", number)),
        }
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            println!("Освобождение документа.");
        }
        unsafe {
            g_object_unref(self.poppler_document as *mut GObject);
        }
    }
}
