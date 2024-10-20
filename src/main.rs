use document::Document;

pub mod ffi;
#[allow(dead_code)]
mod document;
#[allow(dead_code)]
mod page;

fn main() {
    let mut document = Document::new("file:///home/user/test.pdf")
        .expect("Не удалось открыть документ.");

        let total_pages = document.total_pages();

        println!("Total pages: {}", total_pages);

        for i in 0..=3 {
            match document.get_page(i, 2.0) {
                Ok(page) => {
                    let filename = format!("page{}.png", i + 1);
                    if let Err(e) = page.to_png_file(&filename) {
                        println!("Can't save page {} as PNG: {}", i, e);
                    }
                },
                Err(e) => {
                    println!("Error fetching page {}: {}", i, e);
                }
            }
        }
    
}
