# Sitewriter
[![Rust](https://github.com/edg-l/sitewriter/workflows/Rust/badge.svg)](https://github.com/edg-l/sitewriter/actions)
[![crates.io](http://meritbadge.herokuapp.com/sitewriter)](https://crates.io/crates/sitewriter)
[![License](https://img.shields.io/github/license/edg-l/sitewriter)](https://github.com/edg-l/sitewriter/blob/master/LICENSE)
[![codecov](https://codecov.io/gh/edg-l/sitewriter/branch/master/graph/badge.svg?token=JKOQCRSCZU)](https://codecov.io/gh/edg-l/sitewriter)

A rust library to generate sitemaps.

It uses the [quick-xml](https://github.com/tafia/quick-xml) so it should be fast.

## Example

To run the examples use `cargo run --example gen_sitemap`

```rust
 use chrono::prelude::*;
 use sitewriter::*;

let urls = vec![
    UrlEntryBuilder::default()
        .loc("https://edgarluque.com/projects".parse().unwrap())
        .build()
        .unwrap(),
    UrlEntry {
        loc: "https://edgarluque.com/".parse().unwrap(),
        changefreq: Some(ChangeFreq::Daily),
        priority: Some(1.0),
        lastmod: Some(Utc::now()),
    },
    UrlEntry {
        loc: "https://edgarluque.com/blog".parse().unwrap(),
        changefreq: Some(ChangeFreq::Weekly),
        priority: Some(0.8),
        lastmod: Some(Utc::now()),
    },
    UrlEntry {
        loc: "https://edgarluque.com/blog/sitewriter".parse().unwrap(),
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(Utc.ymd(2020, 11, 22).and_hms(15, 10, 15)),
    },
    UrlEntry {
        loc: "https://edgarluque.com/blog/some-future-post"
            .parse()
            .unwrap(),
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(
            Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc()),
        ),
    },
    // Entity escaping
    UrlEntry {
        loc: "https://edgarluque.com/blog/test&id='<test>'"
            .parse()
            .unwrap(),
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(
            Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc()),
        ),
    },
];

let result = Sitemap::into_str(&urls).unwrap();
println!("{}", result);
```

