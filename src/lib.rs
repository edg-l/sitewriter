//! ## A rust library to generate sitemaps.
//! 
//! It uses the [quick-xml](https://github.com/tafia/quick-xml) so it should be fast.
//!
//! To handle the [`Url::lastmod`] tag it uses [chrono](https://docs.rs/chrono/) but it can be disabled with `default-features = false`.
//!
//! ## Example
//!
//! ```rust
//! use chrono::prelude::*;
//! use sitewriter::*;
//! 
//! fn main() {
//!     let mut sitemap = Sitemap::new();
//!     sitemap.urls.push(Url::new("https://edgarluque.com/projects".to_owned()));
//! 
//!     sitemap.urls.push(Url {
//!         loc: "https://edgarluque.com/".to_owned(),
//!         changefreq: Some(ChangeFreq::Daily),
//!         priority: Some(1.0),
//!         lastmod: Some(Utc::now()),
//!     });
//! 
//!     sitemap.urls.push(Url {
//!         loc: "https://edgarluque.com/blog".to_owned(),
//!         changefreq: Some(ChangeFreq::Weekly),
//!         priority: Some(0.8),
//!         lastmod: Some(Utc::now()),
//!     });
//! 
//!     sitemap.urls.push(Url {
//!         loc: "https://edgarluque.com/blog/sitewriter".to_owned(),
//!         changefreq: Some(ChangeFreq::Never),
//!         priority: Some(0.5),
//!         lastmod: Some(Utc.ymd(2020, 11, 22).and_hms(15, 10, 15)),
//!     });
//! 
//!     sitemap.urls.push(Url {
//!         loc: "https://edgarluque.com/blog/some-future-post".to_owned(),
//!         changefreq: Some(ChangeFreq::Never),
//!         priority: Some(0.5),
//!         lastmod: Some(Utc.from_utc_datetime(&Local.ymd(2020, 12, 5).and_hms(12, 30, 0).naive_utc())),
//!     });
//! 
//! 
//!     let result = sitemap.into_str();
//!     println!("{}", result);
//! }
//! ```

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc, SecondsFormat};

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use std::fmt::Display;
use std::io::Cursor;
use std::borrow::Cow;

/// How frequently the page is likely to change. This value provides general information to search engines and may not correlate exactly to how often they crawl the page.
#[derive(Debug)]
pub enum ChangeFreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl Display for ChangeFreq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "montly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        };
        f.write_str(what)
    }
}

/// A sitemap url entry.
#[derive(Debug)]
pub struct Url {
    /// URL of the page.
    ///
    /// This URL must begin with the protocol (such as http) and end with a trailing slash, if your web server requires it. This value must be less than 2,048 characters.
    pub loc: String,
    #[cfg(feature = "chrono")]
    /// The date of last modification of the file.
    pub lastmod: Option<DateTime<Utc>>,
    #[cfg(not(feature = "chrono"))]
    /// The date of last modification of the file.
    ///
    /// This date should be in W3C Datetime format. This format allows you to omit the time portion, if desired, and use YYYY-MM-DD.
    pub lastmod: Option<String>,
    /// How frequently the page is likely to change.
    pub changefreq: Option<ChangeFreq>,
    /// The priority of this URL relative to other URLs on your site. Valid values range from 0.0 to 1.0.
    ///
    /// This value does not affect how your pages are compared to pages on other sites—it only lets the search engines know which pages you deem most important for the crawlers.
    pub priority: Option<f32>,
}

impl Url {
    /// Creates a url (sitemap entry) with only the required elements.
    pub fn new(loc: String) -> Self {
        Self {
            loc,
            lastmod: None,
            changefreq: None,
            priority: None
        }
    }
}

/// Struct to hold the sitemap information.
#[derive(Debug)]
pub struct Sitemap {
    pub urls: Vec<Url>,
}

fn write_tag<T: std::io::Write>(writer: &mut Writer<T>, tag: &str, text: &str) {
    writer
        .write_event(Event::Start(BytesStart::borrowed_name(tag.as_bytes())))
        .expect(&format!("error opening {}", tag));
    writer
        .write_event(Event::Text(BytesText::from_plain_str(text)))
        .expect(&format!("error writing text to {}", tag));
    writer
        .write_event(Event::End(BytesEnd::borrowed(tag.as_bytes())))
        .expect(&format!("error opening {}", tag));
}

impl Sitemap {
    /// Create a new sitemap.    
    pub fn new() -> Self {
        Self {
            urls: Vec::new(),
        }
    }

    /// Generates the sitemap using the provided writer.
    ///
    /// It's recommended to use [`Sitemap::into_bytes()`] or [`Sitemap::into_str()`]
    pub fn generate<T>(&self, inner_writer: T) -> T
    where
        T: std::io::Write,
    {
        let mut writer = Writer::new_with_indent(inner_writer, b' ', 4);
        writer
            .write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))
            .expect("error creating xml decl");

        let urlset_name = b"urlset";
        let mut urlset = BytesStart::borrowed_name(urlset_name);
        urlset.push_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"));
        writer
            .write_event(Event::Start(urlset))
            .expect("error opening urlset");

        for url in self.urls.iter() {
            writer
                .write_event(Event::Start(BytesStart::borrowed_name(b"url")))
                .expect("error opening url");
            write_tag(&mut writer, "loc", &url.loc);

            #[cfg(feature = "chrono")]
            {
                if let Some(lastmod) = &url.lastmod {
                    write_tag(&mut writer, "lastmod", &lastmod.to_rfc3339_opts(SecondsFormat::Secs, true));
                }
            }
            #[cfg(not(feature = "chrono"))]
            {
                if let Some(lastmod) = &url.lastmod {
                    write_tag(&mut writer, "lastmod", lastmod);
                }
            }
            if let Some(priority) = &url.priority {
                write_tag(&mut writer, "priority", &format!("{:.1}", priority))
            }
            if let Some(changefreq) = &url.changefreq {
                write_tag(&mut writer, "changefreq", &changefreq.to_string());
            }

            writer
                .write_event(Event::End(BytesEnd::borrowed(b"url")))
                .expect("error closing url");
        }

        writer
            .write_event(Event::End(BytesEnd::borrowed(urlset_name)))
            .expect("error closing urlset");

        writer.into_inner()
    }

    /// Generates the sitemap.
    pub fn into_bytes(&self) -> Cow<[u8]> {
        let inner = Cursor::new(Vec::new());
        let result = self.generate(inner);
        Cow::Owned(result.into_inner())
    }

    /// Generates the sitemap returning a string.
    pub fn into_str(&self) -> Cow<str> {
        let bytes = self.into_bytes();
        let res = std::str::from_utf8(&bytes).expect("error parsing sitemap bytes to str").to_owned();
        Cow::Owned(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "chrono")]
    #[test]
    fn it_works() {
        use chrono::Utc;

        let mut sitemap = Sitemap::new();
        sitemap.urls.push(Url::new("https://domain.com/".to_owned()));

        sitemap.urls.push(Url {
                loc: "https://domain.com/url".to_owned(),
                changefreq: Some(ChangeFreq::Daily),
                priority: Some(0.8),
                lastmod: Some(Utc::now())
            }
        );

        sitemap.into_str();
    }

    #[cfg(not(feature = "chrono"))]
    #[test]
    fn it_works() {
        let mut sitemap = Sitemap::new();
        sitemap.urls.push(Url::new("https://domain.com/".to_owned()));

        sitemap.urls.push(Url {
                loc: "https://domain.com/url".to_owned(),
                changefreq: Some(ChangeFreq::Daily),
                priority: Some(0.8),
                lastmod: Some("2020-11-22".to_owned()),
            }
        );

        sitemap.into_str();
    }
}