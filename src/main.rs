/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved.
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

mod yaml;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
struct CategoryInfo {
    icon: &'static str,
    description: &'static str,
}

fn info_from_category_name(name: CategoryType) -> CategoryInfo {
    let lookup: std::collections::HashMap<CategoryType, CategoryInfo> = [
        (
            CategoryType::Added,
            CategoryInfo {
                icon: "star2",
                description: "added",
            },
        ),
        (
            CategoryType::Changed,
            CategoryInfo {
                icon: "hammer_and_wrench",
                description: "changed",
            },
        ),
        (
            CategoryType::Fixed,
            CategoryInfo {
                icon: "lady_beetle",
                description: "fixed",
            },
        ),
        (
            CategoryType::Workaround,
            CategoryInfo {
                icon: "see_no_evil",
                description: "workaround",
            },
        ),
        (
            CategoryType::Performance,
            CategoryInfo {
                icon: "zap",
                description: "performance",
            },
        ),
        (
            CategoryType::Tests,
            CategoryInfo {
                icon: "vertical_traffic_light",
                description: "test",
            },
        ),
        (
            CategoryType::Removed,
            CategoryInfo {
                icon: "fire",
                description: "removed",
            },
        ),
        (
            CategoryType::Improved,
            CategoryInfo {
                icon: "art",
                description: "improved",
            },
        ),
        (
            CategoryType::Breaking,
            CategoryInfo {
                icon: "triangular_flag_on_post",
                description: "breaking",
            },
        ),
        (
            CategoryType::Deprecated,
            CategoryInfo {
                icon: "spider_web",
                description: "deprecated",
            },
        ),
        (
            CategoryType::Refactored,
            CategoryInfo {
                icon: "recycle",
                description: "refactor",
            },
        ),
        (
            CategoryType::Experimental,
            CategoryInfo {
                icon: "alembic",
                description: "experimental",
            },
        ),
        (
            CategoryType::Docs,
            CategoryInfo {
                icon: "book",
                description: "docs",
            },
        ),
        (
            CategoryType::Noted,
            CategoryInfo {
                icon: "beetle",
                description: "known issue",
            },
        ),
        (
            CategoryType::Style,
            CategoryInfo {
                icon: "gem",
                description: "style",
            },
        ),
        (
            CategoryType::Unreleased,
            CategoryInfo {
                icon: "soon",
                description: "unreleased",
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let info = lookup.get(&name).unwrap_or_else(|| {
        panic!("unknown '{:?}'", name);
    });

    info.clone()
}

fn print_line(change_type: CategoryType, s: String) {
    let info = info_from_category_name(change_type);
    println!("* :{}: {}", info.icon, s.trim());
}

fn print_optional_list(change_type: CategoryType, list: Option<Vec<String>>) {
    if let Some(items) = list {
        for item in items {
            print_line(change_type.clone(), item)
        }
    }
}

fn print_changes(changes: yaml::Changes) {
    print_optional_list(CategoryType::Added, changes.added);
    print_optional_list(CategoryType::Breaking, changes.breaking);
    print_optional_list(CategoryType::Changed, changes.changed);
    print_optional_list(CategoryType::Deprecated, changes.deprecated);
    print_optional_list(CategoryType::Docs, changes.docs);
    print_optional_list(CategoryType::Experimental, changes.experimental);
    print_optional_list(CategoryType::Fixed, changes.fixed);
    print_optional_list(CategoryType::Improved, changes.improved);
    print_optional_list(CategoryType::Noted, changes.noted);
    print_optional_list(CategoryType::Performance, changes.performance);
    print_optional_list(CategoryType::Refactored, changes.refactored);
    print_optional_list(CategoryType::Removed, changes.removed);
    print_optional_list(CategoryType::Security, changes.security);
    print_optional_list(CategoryType::Style, changes.style);
    print_optional_list(CategoryType::Tests, changes.tests);
    print_optional_list(CategoryType::Unreleased, changes.unreleased);
    print_optional_list(CategoryType::Workaround, changes.workaround);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    eprintln!("Accepting input from stdin");

    let deserialized: yaml::Document = serde_yaml::from_reader(reader).unwrap();

    println!("# Changelog");

    for (release_version, release) in deserialized.releases {
        let link_to_version = format!(
            "https://github.com/{}/releases/tag/{}",
            deserialized.repo, release_version
        );

        println!(
            "\n## :bookmark: [{}]({}) ({})\n",
            release_version, link_to_version, release.date
        );
        if let Some(notice) = release.notice {
            println!("{}", notice.trim());
        }

        for (section_name, section) in release.sections {
            println!("\n### {}\n", section_name.trim());

            if let Some(notice) = section.notice {
                println!("{}\n", notice.trim());
            }

            print_changes(section.changes);
        }
    }
}
