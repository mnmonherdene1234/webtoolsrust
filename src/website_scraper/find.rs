use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use scraper::{Html, Selector};


pub async fn find_attribute_values(page_url: &str, tag: &str, attribute: &str, contains: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Launch the browser and handler
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;

    // Spawn a new task that continuously polls the handler
    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    // Create a new browser page and navigate to the URL
    let page = browser.new_page(page_url).await?;

    // Get the page content (HTML)
    let content = page.content().await?;

    // Parse the HTML content using `scraper`
    let document = Html::parse_document(&content);

    let selector = Selector::parse(tag).unwrap();

    let mut founds: Vec<String> = Vec::new();

    for element in document.select(&selector) {
        if let Some(data) = element.value().attr(attribute) {
            if data.contains(contains) {
                founds.push(data.to_string());
            }
        }
    }

    // Close the browser and wait for the handler task to finish
    browser.close().await?;
    handle.await;

    // Return the vector of URLs
    Ok(founds)
}
