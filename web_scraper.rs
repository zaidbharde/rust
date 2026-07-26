use scraper::{Html, Selector};
use std::collections::HashSet;

#[derive(Debug)]
struct ScrapedPage {
    url:         String,
    title:       Option<String>,
    links:       Vec<String>,
    headings:    Vec<(String, String)>,   
    word_count:  usize,
    image_count: usize,
}

fn scrape(url: &str) -> Result<ScrapedPage, Box<dyn std::error::Error>> {
    println!("  Fetching: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;
    let doc  = Html::parse_document(&body);

    
    let title_sel = Selector::parse("title").unwrap();
    let title = doc.select(&title_sel)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string());

    let a_sel = Selector::parse("a[href]").unwrap();
    let mut seen = HashSet::new();
    let links: Vec<String> = doc.select(&a_sel)
        .filter_map(|e| e.value().attr("href"))
        .filter(|href| href.starts_with("http"))
        .map(String::from)
        .filter(|href| seen.insert(href.clone()))
        .take(20)
        .collect();

    let mut headings = Vec::new();
    for tag in &["h1", "h2", "h3"] {
        let sel = Selector::parse(tag).unwrap();
        for elem in doc.select(&sel) {
            let text = elem.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                headings.push((tag.to_string(), text));
            }
        }
    }

    let p_sel = Selector::parse("p").unwrap();
    let word_count: usize = doc.select(&p_sel)
        .map(|e| e.text().collect::<String>()
            .split_whitespace()
            .count())
        .sum();

    let img_sel   = Selector::parse("img").unwrap();
    let image_count = doc.select(&img_sel).count();

    Ok(ScrapedPage { url: url.to_string(), title, links, headings, word_count, image_count })
}

fn print_report(page: &ScrapedPage) {
    println!("\n{}", "=".repeat(60));
    println!("  📄 Scrape Report");
    println!("{}", "=".repeat(60));
    println!("  URL         : {}", page.url);
    println!("  Title       : {}", page.title.as_deref().unwrap_or("N/A"));
    println!("  Words       : {}", page.word_count);
    println!("  Images      : {}", page.image_count);
    println!("  Links found : {}", page.links.len());

    if !page.headings.is_empty() {
        println!("\n  Headings:");
        for (tag, text) in &page.headings {
            println!("    <{}> {}", tag, text);
        }
    }

    if !page.links.is_empty() {
        println!("\n  Links (first 5):");
        for link in page.links.iter().take(5) {
            println!("    → {}", link);
        }
    }
}

fn main() {
    match scrape("https://www.rust-lang.org") {
        Ok(page)  => print_report(&page),
        Err(e)    => eprintln!("Error: {}", e),
    }
}
