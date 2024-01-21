// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    use wiserone::sitemap::generate_sitemap_file;
    use std::fs;
    use std::error::Error;

    #[test]
    fn test_generate_sitemap_file_no_html_files() -> Result<(), Box<dyn Error>> {
        // Ensure the docs directory is empty
        fs::remove_dir_all("./docs")?;
        fs::create_dir("./docs")?;

        // Expect an empty sitemap to be generated
        generate_sitemap_file("https://example.com/docs/")?;

        let sitemap_content = fs::read_to_string("./docs/sitemap.xml")?;
        assert!(sitemap_content.contains("<urlset xmlns="));
        assert!(!sitemap_content.contains("<loc>"));

        Ok(())
    }
}
