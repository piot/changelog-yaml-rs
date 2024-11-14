/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/changelog-yaml-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::collections::HashMap;
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Changes {
    // Added denotes new features or functionalities introduced in the software.
    pub added: Option<Vec<String>>,

    // Changed indicates changes to existing features or functionalities.
    pub changed: Option<Vec<String>>,

    // Deprecated signifies functionalities that are no longer recommended and will be removed in future versions.
    pub deprecated: Option<Vec<String>>,

    // Removed lists functionalities or features that have been removed from the software. Should have been set as Deprecated in version prior to being removed. Is implicitly breaking changes.
    pub removed: Option<Vec<String>>,

    // Fixed enumerates fixes for bugs or issues in the software.
    pub fixed: Option<Vec<String>>,

    // Security includes changes related to security enhancements or fixes.
    pub security: Option<Vec<String>>,

    // ---------------- Others ---------------

    // Improved lists improvements made to existing functionalities without adding new features.
    pub improved: Option<Vec<String>>,

    // Workaround provides workarounds or temporary solutions for known issues or limitations.
    pub workaround: Option<Vec<String>>,

    // Tests includes changes or additions to testing procedures or test cases.
    pub tests: Option<Vec<String>>,

    // Docs lists changes or additions to documentation, such as README files or inline code comments.
    pub docs: Option<Vec<String>>,

    // Refactored denotes changes made to improve code structure or organization without changing external behavior.
    pub refactored: Option<Vec<String>>,

    // Performance includes changes aimed at improving the performance of the software.
    pub performance: Option<Vec<String>>,

    // Breaking denotes changes that may break backward compatibility with previous versions. Changed, but breaks the API compatibilty.
    pub breaking: Option<Vec<String>>,

    // Experimental lists experimental features or functionalities that are not yet stable or fully supported and might be removed with short or no notice in future versions.
    pub experimental: Option<Vec<String>>,

    // Noted provides a place to note any other significant changes not covered by the above categories.
    pub noted: Option<Vec<String>>,

    // Style denotes changes related to coding style, formatting, or other stylistic aspects.
    pub style: Option<Vec<String>>,

    // Unreleased contains a list of changes that are planned but not yet released in any version.
    // These changes typically represent work that is in progress or pending release in a future version.
    // Once a version is released, the changes listed in Unreleased are moved to the appropriate category (e.g., Added, Changed, Fixed, etc.).
    pub unreleased: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Section {
    pub notice: Option<String>,
    pub changes: Changes,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Release {
    pub date: String,
    pub notice: Option<String>,
    pub repos: Option<IndexMap<String, Changes>>,
    pub sections: Option<IndexMap<String, Section>>,
    pub packages: Option<IndexMap<String, Changes>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DependencyRepoInfo {
    pub repo: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Document {
    pub repo: String,
    pub releases: IndexMap<String, Release>,
    pub repos: Option<HashMap<String, DependencyRepoInfo>>,
}
