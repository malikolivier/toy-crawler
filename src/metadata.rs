use scraper::{Html, Selector};

pub struct Metadata {
    /// How many links are on the page
    pub num_links: usize,
    // How many images are on the page
    pub num_images: usize,
}

/// Get some metadata out of an HTML document
pub fn get_metadata(bytes: &[u8]) -> Metadata {
    // We can afford to make a lossy conversion to UTF-8 before parsing the
    // HTML.  Indeed, we are only parsing elements such as <a> or <img>, and
    // encoding won't be an issue.
    let document = String::from_utf8_lossy(bytes);

    // We assume that the document is a properly formed HTML5
    let html = Html::parse_document(&document);

    // Naively look for links using the <a> tag
    let link_selector = Selector::parse("a").unwrap();
    let num_links = html.select(&link_selector).count();

    // Naively look for images using the <img> tag
    // This ignores all images loaded in CSS for example.
    let image_selector = Selector::parse("img").unwrap();
    let num_images = html.select(&image_selector).count();

    Metadata {
        num_links,
        num_images,
    }
}
