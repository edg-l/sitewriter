# Sitewriter
![Rust](https://github.com/edg-l/sitewriter/workflows/Rust/badge.svg)
[![crates.io](http://meritbadge.herokuapp.com/sitewriter)](https://crates.io/crates/sitewriter)
![License](https://img.shields.io/github/license/edg-l/sitewriter)
[![codecov](https://codecov.io/gh/edg-l/sitewriter/branch/master/graph/badge.svg?token=JKOQCRSCZU)](https://codecov.io/gh/edg-l/sitewriter)

A rust library to generate sitemaps.

It uses the [quick-xml](https://github.com/tafia/quick-xml) so it should be fast.

To handle the `lastmod` tag it uses [chrono](https://docs.rs/chrono/) but it can be disabled with `default-features = false`.


## Example

To run the examples use `cargo run --example gen_sitemap`

```rust
use chrono::prelude::*;
use sitewriter::*;

fn main() {
    let mut sitemap = Sitemap::new();
    sitemap.urls.push(Url::new("https://edgarluque.com/projects".to_owned()));

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/".to_owned(),
        changefreq: Some(ChangeFreq::Daily),
        priority: Some(1.0),
        lastmod: Some(Utc::now()),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog".to_owned(),
        changefreq: Some(ChangeFreq::Weekly),
        priority: Some(0.8),
        lastmod: Some(Utc::now()),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog/sitewriter".to_owned(),
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(Utc.ymd(2020, 11, 22).and_hms(15, 10, 15)),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog/some-future-post".to_owned(),
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc())),
    });


    let result = sitemap.into_str();
    println!("{}", result);
}
```

Prints the following:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://edgarluque.com/projects</loc>
    </url>
    <url>
        <loc>https://edgarluque.com/</loc>
        <lastmod>2020-11-22T14:36:30Z</lastmod>
        <priority>1.0</priority>
        <changefreq>daily</changefreq>
    </url>
    <url>
        <loc>https://edgarluque.com/blog</loc>
        <lastmod>2020-11-22T14:36:30Z</lastmod>
        <priority>0.8</priority>
        <changefreq>weekly</changefreq>
    </url>
    <url>
        <loc>https://edgarluque.com/blog/sitewriter</loc>
        <lastmod>2020-11-22T15:10:15Z</lastmod>
        <priority>0.5</priority>
        <changefreq>never</changefreq>
    </url>
    <url>
        <loc>https://edgarluque.com/blog/some-future-post</loc>
        <lastmod>2020-12-05T11:30:00Z</lastmod>
        <priority>0.5</priority>
        <changefreq>never</changefreq>
    </url>
</urlset>
```
