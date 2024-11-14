/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/changelog-yaml-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::{env, io};
use std::collections::HashMap;
use std::path::PathBuf;
use regex::Regex;
mod formatters;

use crate::formatter::{AdmonitionFormatter, AdmonitionType, EmojiFormatter, HeadingFormatter, LinkFormatter};
use crate::formatters::ascii_doc::AsciiDocFormatter;
use crate::formatters::markdown_github::MarkdownGitHubFormatter;
use crate::yaml::Document;

mod yaml;
mod formatter;
mod emoji;

const GITHUB_URL_PREFIX: &str = "https://github.com/";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CategoryType {
    Changed,
    Added,
    Breaking,
    Deprecated,
    Docs,
    Experimental,
    Fixed,
    Improved,
    Noted,
    Performance,
    Refactored,
    Removed,
    Security,
    Style,
    Tests,
    Unreleased,
    Workaround,
}


fn string_to_admonition_type(name: &str) -> AdmonitionType {
    match name {
        "WARNING" => AdmonitionType::Warning,
        "NOTE" => AdmonitionType::Note,
        "IMPORTANT" => AdmonitionType::Important,
        _ => panic!("{}", format!("unknown admonition: '{}'", name)),
    }
}

fn replace_admonition<F: AdmonitionFormatter>(line: &str, formatter: &F) -> String {
    let re = Regex::new(r#"(WARNING|TIP|NOTE|IMPORTANT|CAUTION):\s.*"#).unwrap();
    let mut line_to_print = String::new();
    let mut previous_match_position = 0;

    for mat in re.find_iter(line) {
        let match_str = mat.as_str();
        let parts: Vec<&str> = match_str.splitn(2, ':').collect();

        line_to_print += &line[previous_match_position..mat.start()];
        line_to_print += &formatter.admonition(string_to_admonition_type(parts[0]), &parts[1][1..]);

        previous_match_position = mat.end();
    }

    line_to_print += &line[previous_match_position..];
    line_to_print
}


fn replace_pull_request_link(
    line: &str,
    repo_short_url: &str,
    formatter: &dyn LinkFormatter,
) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"#\d*").unwrap();
    let mut line_to_print = String::new();
    let mut previous_match_position = 0;

    for mat in re.find_iter(line) {
        let match_str = mat.as_str();
        let pull_request_id = match_str[1..].parse::<usize>()?;
        let suffix = format!("pull/{}", pull_request_id);
        let pull_request_link = format!("{}{}/{}", GITHUB_URL_PREFIX, repo_short_url, suffix);
        let pull_request_complete_link =
            formatter.link(&format!("#{}", pull_request_id), &pull_request_link);
        line_to_print += &line[previous_match_position..mat.start()];
        line_to_print += &pull_request_complete_link;
        previous_match_position = mat.end();
    }

    line_to_print += &line[previous_match_position..];
    Ok(line_to_print)
}

fn replace_commit_hash_link(
    line: &str,
    repo_short_url: &str,
    formatter: &dyn LinkFormatter,
) -> String {
    let re = Regex::new(r"\$[a-f\d]*").unwrap();
    let mut line_to_print = String::new();
    let mut previous_match_position = 0;

    for mat in re.find_iter(line) {
        let match_str = mat.as_str();
        let commit_hash_string = &match_str[1..];
        let commit_hash_link = format!(
            "{}{}/commit/{}",
            GITHUB_URL_PREFIX, repo_short_url, commit_hash_string
        );
        let commit_hash_link_complete = formatter.link(commit_hash_string, &commit_hash_link);
        line_to_print += &line[previous_match_position..mat.start()];
        line_to_print += &commit_hash_link_complete;
        previous_match_position = mat.end();
    }

    line_to_print += &line[previous_match_position..];
    line_to_print
}

fn replace_at_profile_link<F: LinkFormatter>(line: &str, formatter: &F) -> String {
    let re = Regex::new(r"@[\w-]*").unwrap();
    let mut line_to_print = String::new();
    let mut previous_match_position = 0;

    for mat in re.find_iter(line) {
        let match_str = mat.as_str();
        let username_string = &match_str[1..]; // Trim "@" from the match
        let username_profile_link = format!("{}{}", GITHUB_URL_PREFIX, username_string);
        let username_profile_link_complete = formatter.link(match_str, &username_profile_link);
        line_to_print += &line[previous_match_position..mat.start()];
        line_to_print += &username_profile_link_complete;
        previous_match_position = mat.end();
    }

    line_to_print += &line[previous_match_position..];
    line_to_print
}

fn replace_line<F: LinkFormatter>(line: &str, repo_short_url: &str, formatter: &F) -> String {
    let mut s = replace_at_profile_link(line, formatter);
    s = replace_commit_hash_link(&s, repo_short_url, formatter);
    replace_pull_request_link(&s, repo_short_url, formatter)
        .expect("should work to format PR link")
}

fn replace_notice<F: AdmonitionFormatter + LinkFormatter>(line: &str, formatter: &F) -> String {
    let s = replace_admonition(line, formatter);
    replace_at_profile_link(&s, formatter)
}

#[derive(Debug, Clone)]
struct CategoryInfo {
    icon: &'static str,
    description: &'static str,
}


fn description_from_category(category_type: &CategoryType) -> &'static str {
    match category_type {
        CategoryType::Added => "added",
        CategoryType::Changed => "changed",
        CategoryType::Fixed => "fixed",
        CategoryType::Workaround => "workaround",
        CategoryType::Performance => "performance",
        CategoryType::Tests => "test",
        CategoryType::Removed => "removed",
        CategoryType::Improved => "improved",
        CategoryType::Breaking => "breaking",
        CategoryType::Deprecated => "deprecated",
        CategoryType::Refactored => "refactor",
        CategoryType::Experimental => "experimental",
        CategoryType::Docs => "docs",
        CategoryType::Noted => "known issue",
        CategoryType::Style => "style",
        CategoryType::Unreleased => "unreleased",
        CategoryType::Security => "security", // Adding for completeness
    }
}

fn print_line<F: LinkFormatter + EmojiFormatter>(
    repo_url: &str,
    change_type: &CategoryType,
    s: &String,
    formatter: &F,
) {
    let replaced = replace_line(s.trim(), repo_url, formatter);
    let description = description_from_category(change_type);
    if change_type == &CategoryType::Breaking {
        println!("* {}[{}] {}", formatter.emoji(change_type), description, replaced);
    } else {
        println!("* {} {}", formatter.emoji(change_type), replaced);
    }
}

fn print_optional_list<F: LinkFormatter + EmojiFormatter>(
    repo_url: &str,
    change_type: CategoryType,
    list: &Option<Vec<String>>,
    formatter: &F,
) {
    if let Some(items) = list {
        for item in items {
            print_line(repo_url, &change_type, item, formatter)
        }
    }
}


fn print_changes<F: LinkFormatter + EmojiFormatter>(repo_url: &str, changes: &yaml::Changes, formatter: &F) {
    print_optional_list(
        repo_url,
        CategoryType::Unreleased,
        &changes.unreleased,
        formatter,
    );

    print_optional_list(
        repo_url,
        CategoryType::Breaking,
        &changes.breaking,
        formatter,
    );

    print_optional_list(repo_url, CategoryType::Added, &changes.added, formatter);

    print_optional_list(repo_url, CategoryType::Fixed, &changes.fixed, formatter);

    print_optional_list(
        repo_url,
        CategoryType::Workaround,
        &changes.workaround,
        formatter,
    );

    print_optional_list(repo_url, CategoryType::Changed, &changes.changed, formatter);

    print_optional_list(repo_url, CategoryType::Removed, &changes.removed, formatter);

    print_optional_list(
        repo_url,
        CategoryType::Improved,
        &changes.improved,
        formatter,
    );

    print_optional_list(repo_url, CategoryType::Docs, &changes.docs, formatter);

    print_optional_list(repo_url, CategoryType::Tests, &changes.tests, formatter);

    print_optional_list(
        repo_url,
        CategoryType::Refactored,
        &changes.refactored,
        formatter,
    );

    print_optional_list(
        repo_url,
        CategoryType::Deprecated,
        &changes.deprecated,
        formatter,
    );

    print_optional_list(
        repo_url,
        CategoryType::Experimental,
        &changes.experimental,
        formatter,
    );

    print_optional_list(repo_url, CategoryType::Noted, &changes.noted, formatter);

    print_optional_list(
        repo_url,
        CategoryType::Performance,
        &changes.performance,
        formatter,
    );

    print_optional_list(repo_url, CategoryType::Style, &changes.style, formatter);

    print_optional_list(
        repo_url,
        CategoryType::Security,
        &changes.security,
        formatter,
    );
}

fn print_document<F: AdmonitionFormatter + LinkFormatter + HeadingFormatter + EmojiFormatter>(deserialized: Document, formatter: &F) {
    println!("{}", formatter.heading(1, "Changelog"));

    for (release_version, release) in deserialized.releases {
        let link_to_version = format!(
            "https://github.com/{}/releases/tag/{}",
            deserialized.repo, release_version
        );

        let heading = format!("{} {} ({})",
                              formatter.emoji_tag(),
                              formatter.link(&release_version, &link_to_version),
                              release.date);
        println!("\n{}\n", formatter.heading(2, &heading));

        if let Some(notice) = release.notice {
            println!("{}", replace_notice(notice.trim(), formatter));
        }

        if let Some(sections) = release.sections {
            for (section_name, section) in sections {
                println!("\n{}\n", formatter.heading(3, section_name.trim()));

                if let Some(notice) = section.notice {
                    println!("{}\n", replace_notice(notice.trim(), formatter));
                }

                print_changes(&deserialized.repo, &section.changes, formatter);
            }
        }

        if let Some(packages) = release.packages {
            for (package_name, section) in &packages {
                let repo_link = PathBuf::new().join("https://crates.io/crates/").join(package_name);
                let link = formatter.link(&*package_name, repo_link.to_str().unwrap());
                println!("\n{}\n", formatter.heading(3, &*link));

                if let Some(notice) = &section.notice {
                    println!("{}\n", replace_notice(notice.trim(), formatter));
                }

                print_changes(&deserialized.repo, &section.changes, formatter);
            }
        }

        if let Some(repos) = &deserialized.repos {
            if let Some(dependency_repos) = release.repos {
                for (repo_name, changes_in_repo) in dependency_repos {
                    let info = &repos[&repo_name];
                    let repo_url = format!("{}{}", GITHUB_URL_PREFIX, info.repo);
                    let link = formatter.link(&repo_name, &repo_url);
                    let mut description: String = "".to_string();

                    if !info.description.is_empty() {
                        description = format!(" - {}", info.description);
                    }

                    let complete_line = format!("{}{}", link, description);

                    println!("\n{}\n", formatter.heading(3, complete_line.trim()));

                    print_changes(&info.repo, &changes_in_repo, formatter);
                }
            }
        } else {
            continue;
        }
    }
}

fn main() {
    eprintln!("Accepting input from stdin");
    let stdin = io::stdin();
    let reader = stdin.lock();
    let deserialized: Document = serde_yaml::from_reader(reader).unwrap();

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("asciidoc") => {
            let formatter = AsciiDocFormatter {};
            print_document(deserialized, &formatter);
        }
        _ => {
            let formatter = MarkdownGitHubFormatter {};
            print_document(deserialized, &formatter);
        }
    }
}
