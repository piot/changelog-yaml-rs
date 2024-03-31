/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/changelog-yaml-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use crate::formatter::{AdmonitionFormatter, AdmonitionType, EmojiFormatter, HeadingFormatter, LinkFormatter, SuperFormatter};

pub(crate) struct MarkdownGitHubFormatter {}

impl SuperFormatter for MarkdownGitHubFormatter {}

fn admonition_type_to_github_name(admonition_type: AdmonitionType) -> &'static str {
    match admonition_type {
        AdmonitionType::Note => "NOTE",
        AdmonitionType::Important => "IMPORTANT",
        AdmonitionType::Warning => "WARNING",
    }
}

impl AdmonitionFormatter for MarkdownGitHubFormatter {
    fn admonition(&self, admonition_type: AdmonitionType, text: &str) -> String {
        format!(
            "> [!{}]\\\n> {}",
            admonition_type_to_github_name(admonition_type),
            text
        )
    }
}

impl HeadingFormatter for MarkdownGitHubFormatter {
    fn heading(&self, level: usize, name: &str) -> String {
        // Panic if the level is outside the range 1 to 6
        assert!((1..=6).contains(&level), "Level must be between 1 and 6");
        // Repeat the '#' symbol according to the heading level.
        let prefix = "#".repeat(level);
        // Format and return the heading string.
        format!("{} {}", prefix, name)
    }
}

impl LinkFormatter for MarkdownGitHubFormatter {
    fn link(&self, name: &str, link: &str) -> String {
        format!("[{}]({})", name, link)
    }
}

impl EmojiFormatter for MarkdownGitHubFormatter {
    fn emoji(&self, name: &str) -> String {
        format!(":{}:", name)
    }
}