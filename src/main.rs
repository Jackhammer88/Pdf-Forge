use anyhow::{Context, Ok};
use document::Document;

pub mod ffi;
#[allow(dead_code)]
mod document;
#[allow(dead_code)]
mod page;

fn main() -> anyhow::Result<()> {
    let mut document = Document::new("file:///home/aleksey/test.pdf")
        .expect("Не удалось открыть документ.");

        let total_pages = document.total_pages();

        println!("Total pages: {}", total_pages);

        let page = document.get_page(0)
            .context("Can't get page 0")?;

        println!("Saving png file...");
        let filename = format!("page-1.png");
        page.to_png_file(&filename, 10.0)?;
        println!("Saved succesfully to {}", &filename);

        println!("Saving to memory...");
        let bytes = page.to_png_bytes(10.0)?;
        println!("Saved succesfully {} bytes", bytes.len());

        Ok(())    
}
