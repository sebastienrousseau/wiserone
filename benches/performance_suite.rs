// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Comprehensive performance benchmarks for wiserone hot paths

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use std::fs;
use std::hint::black_box;
use std::path::Path;
use tempfile::TempDir;
use wiserone::quotes::{read_quotes_from_file, Quote, Quotes};

/// Generate synthetic quote data for benchmarking
fn generate_synthetic_quotes(count: usize) -> Quotes {
    let mut quotes = Vec::new();
    for i in 0..count {
        quotes.push(Quote {
            id: None,
            pillar: None,
            quote_text: format!("This is test quote number {} with some meaningful content to simulate real quote lengths", i),
            author: format!("Author {}", i),
            date_added: format!("2024-{:02}-{:02}T06:06:06Z", (i % 12) + 1, (i % 28) + 1),
            image_url: "https://example.com/image.webp".to_string(),
        });
    }
    Quotes::new(quotes)
}

/// Write synthetic quotes to JSON file
fn write_json_file(
    path: &Path,
    quotes: &Quotes,
) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(quotes)?;
    fs::write(path, json)
}

/// Write synthetic quotes to CSV file
fn write_csv_file(path: &Path, quotes: &Quotes) -> std::io::Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for quote in &quotes.quotes {
        wtr.serialize(quote)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Benchmark quote file parsing at different scales
fn benchmark_quote_parsing(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000];

    for size in sizes {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("quotes.json");
        let csv_path = temp_dir.path().join("quotes.csv");

        let quotes = generate_synthetic_quotes(size);
        write_json_file(&json_path, &quotes).unwrap();
        write_csv_file(&csv_path, &quotes).unwrap();

        let mut group = c.benchmark_group("quote_parsing");
        let _ = group.throughput(Throughput::Elements(size as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("json", size),
            &json_path,
            |b, path| {
                b.iter(|| {
                    black_box(
                        read_quotes_from_file(path.to_str().unwrap())
                            .unwrap(),
                    )
                })
            },
        );

        let _ = group.bench_with_input(
            BenchmarkId::new("csv", size),
            &csv_path,
            |b, path| {
                b.iter(|| {
                    black_box(
                        read_quotes_from_file(path.to_str().unwrap())
                            .unwrap(),
                    )
                })
            },
        );
    }
}

/// Benchmark quote selection operations
fn benchmark_quote_selection(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000];

    for size in sizes {
        let quotes = generate_synthetic_quotes(size);

        let mut group = c.benchmark_group("quote_selection");
        let _ = group.throughput(Throughput::Elements(size as u64));

        let _ = group.bench_function(
            BenchmarkId::new("random_selection", size),
            |b| {
                b.iter(|| {
                    let mut q = generate_synthetic_quotes(size);
                    let _ = black_box(q.select_random_quote().unwrap());
                })
            },
        );

        let _ = group.bench_function(
            BenchmarkId::new("all_quotes_sorted", size),
            |b| {
                b.iter(|| {
                    black_box(quotes.select_all_quotes().unwrap())
                })
            },
        );
    }
}

/// Benchmark string replacement operations (HTML template processing)
fn benchmark_template_processing(c: &mut Criterion) {
    let template = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>{{title}}</title>
        <meta name="author" content="{{author}}">
        <meta name="description" content="{{description}}">
        <link rel="canonical" href="{{canonical}}">
    </head>
    <body>
        <h1>{{title}}</h1>
        <p>By {{author}}</p>
        <p>Date: {{date}}</p>
        <img src="{{banner}}" alt="{{title}}">
        <p>CDN: {{cdn}}</p>
    </body>
    </html>
    "#
    .repeat(10); // Make template larger to simulate real workload

    let quote = Quote {
        id: None,
        pillar: None,
        quote_text: "This is a test quote with sufficient length to simulate real quote processing".to_string(),
        author: "Test Author".to_string(),
        date_added: "2024-01-01T06:06:06Z".to_string(),
        image_url: "https://example.com/image.webp".to_string(),
    };

    let _ = c.bench_function("template_processing", |b| {
        b.iter(|| {
            let mut result = template.clone();
            result = result.replace("{{title}}", &quote.quote_text);
            result = result.replace("{{author}}", &quote.author);
            result = result.replace(
                "{{date}}",
                quote.date_added.split('T').next().unwrap_or(""),
            );
            result = result.replace("{{banner}}", &quote.image_url);
            result =
                result.replace("{{canonical}}", "https://example.com");
            result =
                result.replace("{{description}}", "Test description");
            result =
                result.replace("{{cdn}}", "https://cdn.example.com");
            black_box(result)
        })
    });
}

/// Benchmark file I/O operations
fn benchmark_file_operations(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let content_sizes = vec![1024, 10240, 102400]; // 1KB, 10KB, 100KB

    for size in content_sizes {
        let content = "x".repeat(size);
        let file_path =
            temp_dir.path().join(format!("test_{}.html", size));

        let mut group = c.benchmark_group("file_operations");
        let _ = group.throughput(Throughput::Bytes(size as u64));

        let _ = group.bench_with_input(
            BenchmarkId::new("write", size),
            &(&file_path, &content),
            |b, (path, content)| {
                b.iter(|| fs::write(path, content).unwrap())
            },
        );

        // Create file for read benchmark
        fs::write(&file_path, &content).unwrap();

        let _ = group.bench_with_input(
            BenchmarkId::new("read", size),
            &file_path,
            |b, path| {
                b.iter(|| black_box(fs::read_to_string(path).unwrap()))
            },
        );
    }
}

/// Benchmark memory allocation patterns
fn benchmark_memory_operations(c: &mut Criterion) {
    let _ = c.bench_function("string_concatenation", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..1000 {
                result.push_str(&format!(
                    "Item {}: This is a test string\n",
                    i
                ));
            }
            black_box(result)
        })
    });

    let _ = c.bench_function("vector_allocation", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(format!("Item {}", i));
            }
            black_box(vec)
        })
    });

    let _ = c.bench_function("vector_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(format!("Item {}", i));
            }
            black_box(vec)
        })
    });
}

criterion_group!(
    performance_suite,
    benchmark_quote_parsing,
    benchmark_quote_selection,
    benchmark_template_processing,
    benchmark_file_operations,
    benchmark_memory_operations
);
criterion_main!(performance_suite);
