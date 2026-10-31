// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use dtt::datetime::DateTime;
use oxml::{Document, EmptyElement, Indent, Newline, SerialiseOptions};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Generates a `sitemap.xml` file for all HTML files in the `./docs` folder.
///
/// This is a thin wrapper over [`generate_sitemap_file_in`] that targets the
/// project's default output directory. Prefer [`generate_sitemap_file_in`] in
/// tests, so that a run never touches the committed `docs/` tree.
pub fn generate_sitemap_file(
    base_url: &str,
) -> Result<(), Box<dyn Error>> {
    generate_sitemap_file_in(base_url, Path::new("./docs"))
}

/// Generates a `sitemap.xml` file for all HTML files in `docs_dir`.
///
/// The sitemap is written to `docs_dir/sitemap.xml`. Every HTML file directly
/// inside `docs_dir` is listed as `{base_url}{file_name}`.
///
/// # Arguments
///
/// * `base_url` - URL prefix each entry is joined onto.
/// * `docs_dir` - directory scanned for `.html` files and written into.
///
/// # Errors
///
/// Returns an error if `docs_dir` cannot be read, or if `sitemap.xml` cannot
/// be created or written.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use wiserone::sitemap::generate_sitemap_file_in;
///
/// // Write into a scratch directory instead of the committed `docs/` tree.
/// generate_sitemap_file_in("https://example.com/", Path::new("/tmp/out"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn generate_sitemap_file_in(
    base_url: &str,
    docs_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let urls = html_urls(base_url, docs_dir)?;
    let sitemap_xml = render_sitemap(&urls, &lastmod_now()?);

    // Write the sitemap to a file
    let mut file = fs::File::create(docs_dir.join("sitemap.xml"))?;
    file.write_all(sitemap_xml.as_bytes())?;

    Ok(())
}

/// The current date and time, as the sitemap's `lastmod` value.
fn lastmod_now() -> Result<String, Box<dyn Error>> {
    // Obtain the current date and time in ISO 8601 format using dtt
    let dt = DateTime::new();
    let iso = dt.format_rfc3339()?;

    // Construct the ISO 8601 date and time string
    Ok(format!(
        "{}-{}-{}T{}:{}:{}{}",
        dt.year(),
        &iso[5..7],
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        dt.offset()
    ))
}

/// `{base_url}{file_name}` for every HTML file directly in `docs_dir`.
fn html_urls(
    base_url: &str,
    docs_dir: &Path,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = Vec::new();
    if !docs_dir.exists() {
        return Ok(urls);
    }
    for entry in fs::read_dir(docs_dir)? {
        let path = entry?.path();
        let is_html = path.is_file()
            && path.extension().and_then(|s| s.to_str())
                == Some("html");
        // Safely extract the filename, skipping files with invalid
        // names rather than panicking on them.
        if let (true, Some(file_name)) =
            (is_html, path.file_name().and_then(|n| n.to_str()))
        {
            urls.push(format!("{}{}", base_url, file_name));
        }
    }
    Ok(urls)
}

/// The sitemap document listing `urls`, each with `lastmod`.
fn render_sitemap(urls: &[String], lastmod: &str) -> String {
    const SITEMAP_NS: &str =
        "http://www.sitemaps.org/schemas/sitemap/0.9";
    let mut doc = Document::empty();
    let Ok(urlset) =
        doc.append_element(doc.root(), Some(SITEMAP_NS), "urlset")
    else {
        return String::new();
    };
    let _ = doc.declare_namespace(urlset, None, SITEMAP_NS);
    for (prefix, uri) in [
        ("news", "http://www.google.com/schemas/sitemap-news/0.9"),
        ("xhtml", "http://www.w3.org/1999/xhtml"),
        ("mobile", "http://www.google.com/schemas/sitemap-mobile/1.0"),
        ("image", "http://www.google.com/schemas/sitemap-image/1.1"),
        ("video", "http://www.google.com/schemas/sitemap-video/1.1"),
    ] {
        let _ = doc.declare_namespace(urlset, Some(prefix), uri);
    }

    for url in urls {
        let Ok(url_node) =
            doc.append_element(urlset, Some(SITEMAP_NS), "url")
        else {
            continue;
        };
        for (tag, val) in [
            ("loc", url.as_str()),
            ("changefreq", "weekly"),
            ("lastmod", lastmod),
        ] {
            if let Ok(elem) =
                doc.append_element(url_node, Some(SITEMAP_NS), tag)
            {
                let _ = doc.append_text(elem, val);
            }
        }
    }

    let options = SerialiseOptions {
        indent: Some(Indent::Spaces(2)),
        empty_elements: EmptyElement::Expanded,
        newline: Newline::Lf,
    };
    let mut sitemap_xml =
        String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let _ = doc.write_xml_with(&mut sitemap_xml, options);
    sitemap_xml
}
