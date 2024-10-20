use std::{ffi::{c_double, c_void, CStr, CString}, io::Write, slice};

use anyhow::{bail, Context, Result};

use crate::ffi::{
    cairo::*,
    glib::{g_object_unref, GObject},
    poppler::{poppler_page_get_size, poppler_page_render, PopplerPage},
};

pub struct Page {
    poppler_page: *mut PopplerPage,
    buffer: ByteBuffer,
}

impl Page {
    pub fn new(poppler_page: *mut PopplerPage, scale: f64) -> Result<Page> {
        let buffer = Page::render_png_to_bytes(poppler_page, scale)?;

        Ok(Page {
            poppler_page,
            buffer,
        })
    }

    // Возвращает данные буфера в виде вектора байтов
    pub fn to_memory_box(&self) -> Box<[u8]> {
        unsafe {
            // Преобразуем указатель на данные в срез
            let data_slice = slice::from_raw_parts(self.buffer.data, self.buffer.size);
            // Преобразуем срез в Box<[u8]> для управления памятью
            data_slice.to_vec().into_boxed_slice()
        }
    }

    // Сохраняет данные буфера в файл
    pub fn to_png_file(&self, filename: &str) -> Result<()> {
        let mut file = std::fs::File::create(filename)?;
        // Преобразуем указатель на данные в срез
        unsafe {
            let data_slice = slice::from_raw_parts(self.buffer.data, self.buffer.size);
            file.write_all(data_slice)?;
        }

        Ok(())
    }

    fn render_page_to_png(page: *mut PopplerPage, filename: &str, scale: f64) -> Result<()> {
        let c_filename =
            CString::new(filename).context("Can't get correct file name to pdf document.")?;
    
        let mut width: c_double = 0.0;
        let mut height: c_double = 0.0;
    
        // Получение размеров страницы
        unsafe {
            poppler_page_get_size(
                page as *mut PopplerPage,
                &mut width as *mut c_double,
                &mut height as *mut c_double,
            );
        }
    
        // Увеличение размеров страницы на заданный масштаб
        let scaled_width: i32 = (width * scale) as i32;
        let scaled_height: i32 = (height * scale) as i32;
    
        // Создание Cairo поверхности
        let surface = unsafe {
            let _surface = cairo_image_surface_create(CairoFormat::Argb32, scaled_width, scaled_height);
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
            poppler_page_render(page, cr);
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
    
    
    fn render_png_to_bytes(
        page: *mut PopplerPage, 
        scale: f64) -> 
            Result<ByteBuffer> {
        let mut width: c_double = 0.0;
        let mut height: c_double = 0.0;
    
        // Получение размеров страницы
        unsafe {
            poppler_page_get_size(
                page as *mut PopplerPage,
                &mut width as *mut c_double,
                &mut height as *mut c_double,
            );
        }
    
        // Увеличение размеров страницы на заданный масштаб
        let scaled_width: i32 = (width * scale) as i32;
        let scaled_height: i32 = (height * scale) as i32;
    
        // Создание Cairo поверхности
        let surface = unsafe {
            let _surface = cairo_image_surface_create(CairoFormat::Argb32, scaled_width, scaled_height);
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
            poppler_page_render(page, cr);
            // Завершаем операции на поверхности
            cairo_surface_flush(surface);
        }
    
        let mut buffer = ByteBuffer {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        };
    
        let status = unsafe {
            // Запись данных PNG в память
            cairo_surface_write_to_png_stream(
                surface, write_to_memory,
                &mut buffer as *mut _ as *mut c_void)
        };
    
        if status != cairo_status_t::CAIRO_STATUS_SUCCESS {
            if !buffer.data.is_null() {
                unsafe {
                    libc::free(buffer.data as *mut c_void);
                }
            }
            bail!("Failed to write PNG to memory: {:?}", status);
        }
    
        Ok(buffer)
    }
    
    fn free_byte_buffer(buffer: &mut ByteBuffer) {
        unsafe {
            if !buffer.data.is_null() {
                libc::free(buffer.data as *mut c_void); // Освобождаем выделенную память
                buffer.data = std::ptr::null_mut();     // Устанавливаем указатель в null для безопасности
                buffer.size = 0;                        // Сбрасываем размер буфера
                buffer.capacity = 0;                    // Сбрасываем емкость буфера
            }
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe {
            #[cfg(debug_assertions)]
            {
                println!("Освобождение буфера");
            }
            Page::free_byte_buffer(&mut self.buffer);
            #[cfg(debug_assertions)]
            {
                println!("Освобождение страницы");
            }            
            g_object_unref(self.poppler_page as *mut GObject)
        };
    }
}
