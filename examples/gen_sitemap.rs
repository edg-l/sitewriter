use chrono::prelude::*;
use sitewriter::{ChangeFreq, UrlEntry, UrlEntryBuilder};

fn main() {
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

    let result = sitewriter::generate_str(&urls);
    println!("{}", result);
}
