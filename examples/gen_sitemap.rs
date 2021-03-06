use chrono::prelude::*;
use sitewriter::*;

fn main() {
    let mut sitemap = Sitemap::new();
    sitemap
        .urls
        .push(Url::new("https://edgarluque.com/projects"));

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/",
        changefreq: Some(ChangeFreq::Daily),
        priority: Some(1.0),
        lastmod: Some(Utc::now()),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog",
        changefreq: Some(ChangeFreq::Weekly),
        priority: Some(0.8),
        lastmod: Some(Utc::now()),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog/sitewriter",
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(Utc.ymd(2020, 11, 22).and_hms(15, 10, 15)),
    });

    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog/some-future-post",
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(
            Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc()),
        ),
    });

    // Entity escaping
    sitemap.urls.push(Url {
        loc: "https://edgarluque.com/blog/test&id='<test>'",
        changefreq: Some(ChangeFreq::Never),
        priority: Some(0.5),
        lastmod: Some(
            Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc()),
        ),
    });

    let result = sitemap.into_str();
    println!("{}", result);
}
