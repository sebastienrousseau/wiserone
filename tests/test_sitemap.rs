// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests for sitemap generation (`wiserone::sitemap`).

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;
    use std::process;
    use wiserone::sitemap::generate_sitemap_file_in;

    /// A scratch directory unique to this process and test name.
    ///
    /// These tests must never point at `./docs`: that directory is
    /// committed (66 tracked files) and is what GitHub Pages serves, so
    /// a test writing or clearing it leaves the working tree dirty and
    /// invites an accidental `git add -A` of the deletions.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "wiserone-{}-{}",
            name,
            process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    #[test]
    fn test_generate_sitemap_file_no_html_files(
    ) -> Result<(), Box<dyn Error>> {
        let dir = scratch_dir("sitemap-empty");

        generate_sitemap_file_in("https://example.com/docs/", &dir)?;

        let sitemap_content =
            fs::read_to_string(dir.join("sitemap.xml"))?;
        assert!(sitemap_content.contains("<urlset xmlns="));
        assert!(!sitemap_content.contains("<loc>"));

        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn test_generate_sitemap_file_lists_html_files(
    ) -> Result<(), Box<dyn Error>> {
        let dir = scratch_dir("sitemap-populated");
        fs::write(dir.join("2024_01_01.html"), "<html></html>")?;
        fs::write(dir.join("not-a-page.txt"), "ignored")?;

        generate_sitemap_file_in("https://example.com/docs/", &dir)?;

        let sitemap_content =
            fs::read_to_string(dir.join("sitemap.xml"))?;
        assert!(sitemap_content.contains(
            "<loc>https://example.com/docs/2024_01_01.html</loc>"
        ));
        // Non-HTML files are not listed.
        assert!(!sitemap_content.contains("not-a-page.txt"));
        // The sitemap itself is not listed as a page.
        assert!(!sitemap_content.contains(
            "<loc>https://example.com/docs/sitemap.xml</loc>"
        ));

        fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
