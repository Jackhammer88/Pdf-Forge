use std::ffi::{c_double, c_void, CStr, CString};
use anyhow::{bail, Context, Result};

use crate::ffi::{
    cairo::*,
    glib::{g_object_unref, GObject},
    poppler::{poppler_page_get_size, poppler_page_render, PopplerPage},
};

pub struct Page {
    poppler_page: *mut PopplerPage,
}

impl Page {
    pub fn new(poppler_page: *mut PopplerPage) -> Result<Page> {
        Ok(Page { poppler_page })
    }

    pub fn to_png_file(&self, filename: &str, scale: f64) -> Result<()> {
        let c_filename =
            CString::new(filename).context("Can't get correct file name to pdf document.")?;

        let mut width: c_double = 0.0;
        let mut height: c_double = 0.0;

        // Получение размеров страницы
        unsafe {
            poppler_page_get_size(
                self.poppler_page,
                &mut width as *mut c_double,
                &mut height as *mut c_double,
            );
        }

        // Увеличение размеров страницы на заданный масштаб
        let scaled_width: i32 = (width * scale) as i32;
        let scaled_height: i32 = (height * scale) as i32;

        // Создание Cairo поверхности
        let surface = unsafe {
            let _surface =
                cairo_image_surface_create(CairoFormat::Argb32, scaled_width, scaled_height);
            if _surface.is_null() {
                bail!("Failed to create Cairo surface: surface is null");
            }
            let status = cairo_surface_status(_surface);
            if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
                bail!("Failed to create Cairo surface");
            }
            _surface
        };

        // Создание контекста Cairo
        let cr = unsafe {
            let cr = cairo_create(surface);
            let status = cairo_status(cr);
            if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
                cairo_surface_destroy(surface);
                bail!("Failed to create Cairo context\n");
            }
            cr
        };

        unsafe {
            // Масштабирование контекста
            cairo_scale(cr, scale, scale);
            // Рендер страницы Poppler на поверхность Cairo
            poppler_page_render(self.poppler_page, cr);
            // Завершаем операции на поверхности
            cairo_surface_flush(surface);

            // Сохранение поверхности в файл PNG
            let status = cairo_surface_write_to_png(surface, c_filename.as_ptr());
            if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
                cairo_destroy(cr);
                cairo_surface_destroy(surface);

                let status_str = cairo_status_to_string(status);
                if !status_str.is_null() {
                    let error_message = CStr::from_ptr(status_str).to_string_lossy();
                    bail!(format!("Failed to write PNG: {}", error_message));
                } else {
                    bail!("Failed to write PNG, unknown error.");
                }
            }

            cairo_destroy(cr);
            cairo_surface_destroy(surface);
        }

        Ok(())
    }

    pub fn to_png_bytes(&self, scale: f64) -> Result<Vec<u8>> {
        let mut width: c_double = 0.0;
        let mut height: c_double = 0.0;

        // Get page size
        unsafe {
            poppler_page_get_size(
                self.poppler_page,
                &mut width as *mut c_double,
                &mut height as *mut c_double,
            );
        }

        // Scale page dimensions
        let scaled_width: i32 = (width * scale) as i32;
        let scaled_height: i32 = (height * scale) as i32;

        // Create Cairo surface
        let surface = unsafe {
            let _surface =
                cairo_image_surface_create(CairoFormat::Argb32, scaled_width, scaled_height);
            if _surface.is_null() {
                bail!("Failed to create Cairo surface: surface is null");
            }
            let status = cairo_surface_status(_surface);
            if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
                bail!("Failed to create Cairo surface");
            }
            _surface
        };

        // Create Cairo context
        let cr = unsafe {
            let cr = cairo_create(surface);
            let status = cairo_status(cr);
            if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
                cairo_surface_destroy(surface);
                bail!("Failed to create Cairo context\n");
            }
            cr
        };

        let mut buffer = vec![];

        let status = unsafe {
            // Scale context
            cairo_scale(cr, scale, scale);
            // Render Poppler page onto Cairo surface
            poppler_page_render(self.poppler_page, cr);
            // Flush the surface
            cairo_surface_flush(surface);

            // Write PNG data to memory
            cairo_surface_write_to_png_stream(
                surface,
                write_to_memory,
                &mut buffer as *mut _ as *mut c_void,
            )
        };

        // Properly destroy Cairo context and surface
        unsafe {
            cairo_destroy(cr);
            cairo_surface_destroy(surface);
        }

        if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
            bail!("Failed to write PNG to memory: {:?}", status);
        }

        Ok(buffer)
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe {
            #[cfg(debug_assertions)]
            {
                println!("Освобождение страницы");
            }
            g_object_unref(self.poppler_page as *mut GObject)
        };
    }
}
