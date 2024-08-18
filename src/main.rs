use std::env;

mod website_scraper;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len >= 5 {
        if args[1] == "find_attribute_values" {
            let page_url = &args[2];
            let tag = &args[3];
            let attribute = &args[4];
            let mut contains = "";

            if args_len >= 6 {
                contains = &args[5];
            }

            match website_scraper::find::find_attribute_values(page_url, tag, attribute, contains).await {
                Ok(urls) => {
                    println!("Found the following URLs:");
                    for url in urls {
                        println!("{}", url);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    } else {
        println!("help to print commands");
    }

    Ok(())
}
