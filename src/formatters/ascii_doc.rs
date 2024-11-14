/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/changelog-yaml-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::collections::HashMap;
use crate::CategoryType;
use crate::emoji::utf8_icon;
use crate::formatter::{AdmonitionFormatter, AdmonitionType, EmojiFormatter, HeadingFormatter, LinkFormatter, SuperFormatter};

pub(crate) struct AsciiDocFormatter {}

impl SuperFormatter for AsciiDocFormatter {}

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


impl EmojiFormatter for AsciiDocFormatter {
    fn emoji(&self, category_type: &CategoryType) -> String {
        let unicode_str = utf8_icon(category_type);
        format!("&#x{};", unicode_str)
    }

    fn emoji_tag(&self) -> String {
        "bookmark".to_string()
    }
}
