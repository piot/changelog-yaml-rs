use std::collections::HashMap;
use crate::formatter::{AdmonitionFormatter, AdmonitionType, EmojiFormatter, HeadingFormatter, LinkFormatter};

pub(crate) struct AsciiDocFormatter {}

fn admonition_type_to_asciidoc_keyword(admonition_type: AdmonitionType) -> &'static str {
    match admonition_type {
        AdmonitionType::Note => "NOTE",
        AdmonitionType::Important => "IMPORTANT",
        AdmonitionType::Warning => "WARNING",
    }
}

impl AdmonitionFormatter for AsciiDocFormatter {
    fn admonition(&self, admonition_type: AdmonitionType, text: &str) -> String {
        format!(
            "{}: {}",
            admonition_type_to_asciidoc_keyword(admonition_type),
            text
        )
    }
}

impl HeadingFormatter for AsciiDocFormatter {
    fn heading(&self, level: usize, name: &str) -> String {
        // AsciiDoc uses a single '=' for top-level headings and adds one more '=' for each sub-level.
        // Panic if the level is outside the range 1 to 6
        assert!((1..=6).contains(&level), "Level must be between 1 and 6");
        let prefix = "=".repeat(level);
        format!("{} {}", prefix, name)
    }
}

impl LinkFormatter for AsciiDocFormatter {
    fn link(&self, name: &str, link: &str) -> String {
        format!("link:{}[{}]", link, name)
    }
}


fn emoji_name_to_unicode(name: &str) -> u32 {
    let lookup: HashMap<&str, u32> = [
        ("bookmark", 0x1F516),
        ("triangular_flag_on_post", 0x1f6a9),
        ("star2", 0x1f31f),
        ("hammer_and_wrench", 0x1f6e0),
        ("lady_beetle", 0x1f41e),
        ("see_no_evil", 0x1f648),
        ("zap", 0x26a1),
        ("vertical_traffic_light", 0x1f6a6),
        ("fire", 0x1f525),
        ("art", 0x1f3a8),
        ("spider_web", 0x1f578),
        ("recycle", 0x267b),
        ("alembic", 0x2697),
        ("book", 0x1F4D6),
        ("noted", 0x1FAB2),
        ("gem", 0x1F48E),
        ("soon", 0x1F51C),
    ].iter().cloned().collect();

    *lookup.get(name).expect(&format!("cannot replace {}", name))
}

impl EmojiFormatter for AsciiDocFormatter {
    fn emoji(&self, name: &str) -> String {
        let unicode_int = emoji_name_to_unicode(name);
        format!("&#x{:X};", unicode_int)
    }
}