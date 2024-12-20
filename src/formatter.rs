/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/changelog-yaml-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use crate::CategoryType;

pub(crate) enum AdmonitionType {
    Warning,
    Note,
    Important,
}

pub(crate) trait AdmonitionFormatter {
    fn admonition(&self, ad_type: AdmonitionType, content: &str) -> String;
}

pub(crate) trait LinkFormatter {
    fn link(&self, name: &str, link: &str) -> String;
}

pub(crate) trait EmojiFormatter {
    fn emoji(&self, category_type: &CategoryType) -> String;
    fn emoji_tag(&self) -> String;
}

pub(crate) trait HeadingFormatter {
    fn heading(&self, level: usize, name: &str) -> String;
}

#[allow(unused)]
pub(crate) trait SuperFormatter: AdmonitionFormatter + LinkFormatter + HeadingFormatter + EmojiFormatter {}
