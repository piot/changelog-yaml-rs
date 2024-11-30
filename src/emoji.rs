use crate::CategoryType;

#[allow(unused)]
pub fn github_icon(category_type: &CategoryType) -> &'static str {
    match category_type {
        CategoryType::Changed => ":hammer_and_wrench:", // 🛠️
        CategoryType::Added => ":star2:",               // 🌟
        CategoryType::Breaking => ":triangular_flag_on_post:", // 🚩
        CategoryType::Deprecated => ":spider_web:",     // 🕸️
        CategoryType::Docs => ":book:",                 // 📖
        CategoryType::Experimental => ":alembic:",      // ⚗️
        CategoryType::Fixed => ":lady_beetle:",         // 🐞
        CategoryType::Improved => ":art:",              // 🎨
        CategoryType::Noted => ":warning:",             // ⚠️
        CategoryType::Optimization => ":zap:",          // ⚡
        CategoryType::Refactored => ":recycle:",        // ♻️
        CategoryType::Removed => ":fire:",              // 🔥
        CategoryType::Security => ":lock:",             // 🔒
        CategoryType::Style => ":gem:",                 // 💎
        CategoryType::Tests => ":vertical_traffic_light:", // 🚦
        CategoryType::Unreleased => ":soon:",           // 🔜
        CategoryType::Workaround => ":see_no_evil:",    // 🙈
        CategoryType::Examples => ":scroll:",           // 📜
    }
}

pub fn utf8_icon(category_type: &CategoryType) -> &'static str {
    match category_type {
        CategoryType::Changed => "🛠️",      // hammer_and_wrench
        CategoryType::Added => "✨",        // star2
        CategoryType::Breaking => "🚩",     // triangular_flag_on_post
        CategoryType::Deprecated => "🕸️",   // spider_web
        CategoryType::Docs => "📖",         // book
        CategoryType::Experimental => "⚗️", // alembic
        CategoryType::Fixed => "🐞",        // lady_beetle
        CategoryType::Improved => "🎨",     // art
        CategoryType::Noted => "⚠️",        // exclamation mark in triangle
        CategoryType::Optimization => "⚡", // zap
        CategoryType::Refactored => "♻️",   // recycle
        CategoryType::Removed => "🔥",      // fire
        CategoryType::Security => "🔒",     // lock
        CategoryType::Style => "💎",        // gem
        CategoryType::Tests => "🚦",        // vertical_traffic_light
        CategoryType::Unreleased => "🔜",   // soon
        CategoryType::Workaround => "🙈",   // see no evil
        CategoryType::Examples => "📜",     // scroll
    }
}
