# sitewriter

### A library to generate sitemaps.

[![Version](https://img.shields.io/crates/v/sitewriter)](https://crates.io/crates/sitewriter)
[![Downloads](https://img.shields.io/crates/d/sitewriter)](https://crates.io/crates/sitewriter)
[![License](https://img.shields.io/crates/l/sitewriter)](https://crates.io/crates/sitewriter)
![Rust](https://github.com/edg-l/sitewriter/workflows/Rust/badge.svg)
[![Docs](https://docs.rs/sitewriter/badge.svg)](https://docs.rs/sitewriter)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/sitewriter.svg)](https://web.crev.dev/rust-reviews/crate/sitewriter/)

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

### CREV - Rust code reviews - Raise awareness

Please, spread this info !\
Open source code needs a community effort to express trustworthiness.\
Start with reading the reviews of the crates you use on [web.crev.dev/rust-reviews/crates/](https://web.crev.dev/rust-reviews/crates/) \
Than install the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\. Follow the [Getting Started guide](https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md). \
On your Rust project, verify the trustworthiness of all dependencies, including transient dependencies with `cargo crev verify`\
Write a new review ! \
Describe the crates you trust. Or warn about the crate versions you think are dangerous.\
Help other developers, inform them and share your opinion.\
Use [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) to write reviews easily.

License: MIT
