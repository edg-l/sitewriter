use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sitewriter::*;

fn benchmark(c: &mut Criterion) {
    use chrono::Utc;

    let urls = vec![
        // Builder pattern
        UrlEntryBuilder::default()
            .loc("https://domain.com".parse().unwrap())
            .priority(0.2)
            .build()
            .unwrap(),
        // Using new
        UrlEntry::new(
            "https://domain.com/some_url".parse().unwrap(),
            None,
            None,
            None,
        ),
        // Initializing the struct.
        UrlEntry {
            loc: "https://domain.com/another".parse().unwrap(),
            priority: None,
            changefreq: Some(ChangeFreq::Always),
            lastmod: None,
        },
        UrlEntry {
            loc: "https://domain.com/url".parse().unwrap(),
            changefreq: Some(ChangeFreq::Daily),
            priority: Some(0.8),
            lastmod: Some(Utc::now()),
        },
        UrlEntry {
            loc: "https://domain.com/aa".parse().unwrap(),
            changefreq: Some(ChangeFreq::Monthly),
            priority: None,
            lastmod: None,
        },
        UrlEntry {
            loc: "https://domain.com/bb".parse().unwrap(),
            changefreq: None,
            priority: None,
            lastmod: None,
        },
        UrlEntry {
            loc: "https://domain.com/bb&id='<test>'".parse().unwrap(),
            changefreq: None,
            priority: Some(0.4),
            lastmod: None,
        },
    ];

    c.bench_function("generate_str", |b| {
        b.iter(|| Sitemap::generate_str(black_box(&urls)))
    });

    c.bench_function("generate_bytes", |b| {
        b.iter(|| Sitemap::generate_bytes(black_box(&urls)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
