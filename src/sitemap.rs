// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use dtt::DateTime;

/// Generates a sitemap.xml file for all HTML files in the docs folder.
pub fn generate_sitemap_file(base_url: &str) -> Result<(), Box<dyn Error>> {
    let docs_path = Path::new("./docs");
    let mut urls = Vec::new();

    // Obtain the current date and time in ISO 8601 format using dtt
    let dt = DateTime::new();
    let iso = dt.iso_8601;
    let year_str = dt.year;
    let month_str = &iso[5..7];
    let day_str = dt.day;
    let hour_str = dt.hour.to_string();
    let minute_str = dt.minute.to_string();
    let second_str = dt.second.to_string();
    let offset = dt.offset;

    // Construct the ISO 8601 date and time string
    let iso_8601 = format!("{}-{}-{}T{}:{}:{}{}", year_str, month_str, day_str, hour_str, minute_str, second_str, offset);

    let current_iso_date = iso_8601;
    print!("Current date and time in ISO 8601 format: {}\n", current_iso_date);

    // Collect HTML filenames
    if docs_path.exists() {
        for entry in fs::read_dir(docs_path)? {
            let path = entry?.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("html") {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                urls.push(format!("{}{}", base_url, file_name));
            }
        }
    }

    // Start the XML string with namespaces
    let mut sitemap_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    sitemap_xml += "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" ";
    sitemap_xml += "xmlns:news=\"http://www.google.com/schemas/sitemap-news/0.9\" ";
    sitemap_xml += "xmlns:xhtml=\"http://www.w3.org/1999/xhtml\" ";
    sitemap_xml += "xmlns:mobile=\"http://www.google.com/schemas/sitemap-mobile/1.0\" ";
    sitemap_xml += "xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\" ";
    sitemap_xml += "xmlns:video=\"http://www.google.com/schemas/sitemap-video/1.1\">\n";

    // Add URLs to the sitemap with changefreq and dynamic lastmod
    for url in urls {
        sitemap_xml.push_str(&format!("  <url>\n    <loc>{}</loc>\n", url));
        sitemap_xml.push_str("    <changefreq>weekly</changefreq>\n");
        sitemap_xml.push_str(&format!("    <lastmod>{}</lastmod>\n", current_iso_date));
        sitemap_xml.push_str("  </url>\n");
    }

    // Close the XML string
    sitemap_xml.push_str("</urlset>");

    // Write the sitemap to a file
    let mut file = fs::File::create("./docs/sitemap.xml")?;
    file.write_all(sitemap_xml.as_bytes())?;

    Ok(())
}