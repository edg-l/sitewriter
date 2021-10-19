# sitewriter

### A library to generate sitemaps.

[![Version](https://img.shields.io/crates/v/sitewriter)](https://crates.io/crates/sitewriter)
[![Downloads](https://img.shields.io/crates/d/sitewriter)](https://crates.io/crates/sitewriter)
[![License](https://img.shields.io/crates/l/sitewriter)](https://crates.io/crates/sitewriter)
![Rust](https://github.com/edg-l/sitewriter-rs/workflows/Rust/badge.svg)
[![Docs](https://docs.rs/sitewriter/badge.svg)](https://docs.rs/sitewriter)

It uses the [quick-xml](https://github.com/tafia/quick-xml) so it should be fast.

### Example

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

   let result = Sitemap::generate_str(&urls).unwrap();
   println!("{}", result);
```

License: MIT
