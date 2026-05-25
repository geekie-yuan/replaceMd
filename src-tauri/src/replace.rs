//! 纯文本替换逻辑：构建正则、执行替换、生成预览片段。
//! 这里不碰文件系统，方便单元测试，也便于以后扩展成多条规则。

use regex::{Regex, RegexBuilder};
use serde::Serialize;

/// 预览用的单行片段：命中所在行号，以及替换前/后的整行文本。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Snippet {
    pub line_no: usize,
    pub before: String,
    pub after: String,
}

/// 根据查找内容构建正则。
/// - `is_regex == false`：把查找内容当字面量（转义后匹配）。
/// - `case_sensitive == false`：忽略大小写。
/// 查找内容为空、或正则非法时返回友好错误信息。
pub fn build_regex(find: &str, is_regex: bool, case_sensitive: bool) -> Result<Regex, String> {
    if find.is_empty() {
        return Err("查找内容不能为空".to_string());
    }
    let pattern = if is_regex {
        find.to_string()
    } else {
        regex::escape(find)
    };
    RegexBuilder::new(&pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .map_err(|e| format!("正则表达式无效：{e}"))
}

/// 在整段内容上执行替换，返回 (新内容, 命中次数)。
/// 字面量模式下用 `NoExpand`，避免替换文本里的 `$1`/`$name` 被当成捕获组展开。
pub fn apply_rule(content: &str, re: &Regex, replace: &str, is_regex: bool) -> (String, usize) {
    let count = re.find_iter(content).count();
    let new = if is_regex {
        re.replace_all(content, replace).into_owned()
    } else {
        re.replace_all(content, regex::NoExpand(replace)).into_owned()
    };
    (new, count)
}

/// 分析一段内容：返回 (整段命中次数, 逐行命中片段)。
/// 命中次数按整段内容统计（准确）；片段按行收集（用于界面高亮），最多 `max_snippets` 条。
pub fn analyze(
    content: &str,
    re: &Regex,
    replace: &str,
    is_regex: bool,
    max_snippets: usize,
) -> (usize, Vec<Snippet>) {
    let count = re.find_iter(content).count();
    let mut snippets = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if snippets.len() >= max_snippets {
            break;
        }
        if re.is_match(line) {
            let after = if is_regex {
                re.replace_all(line, replace).into_owned()
            } else {
                re.replace_all(line, regex::NoExpand(replace)).into_owned()
            };
            snippets.push(Snippet {
                line_no: i + 1,
                before: line.to_string(),
                after,
            });
        }
    }
    (count, snippets)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn re(find: &str, is_regex: bool, cs: bool) -> Regex {
        build_regex(find, is_regex, cs).unwrap()
    }

    #[test]
    fn literal_replace_counts_and_replaces() {
        let r = re("foo", false, true);
        let (out, n) = apply_rule("foo bar foo", &r, "baz", false);
        assert_eq!(out, "baz bar baz");
        assert_eq!(n, 2);
    }

    #[test]
    fn no_match_leaves_content_unchanged() {
        let r = re("xyz", false, true);
        let (out, n) = apply_rule("foo bar", &r, "baz", false);
        assert_eq!(out, "foo bar");
        assert_eq!(n, 0);
    }

    #[test]
    fn regex_replace_with_digits() {
        let r = re(r"\d+", true, true);
        let (out, n) = apply_rule("a1 b22 c333", &r, "#", true);
        assert_eq!(out, "a# b# c#");
        assert_eq!(n, 3);
    }

    #[test]
    fn case_insensitive_matches_mixed_case() {
        let r = re("foo", false, false);
        let (out, n) = apply_rule("Foo FOO foo", &r, "x", false);
        assert_eq!(out, "x x x");
        assert_eq!(n, 3);
    }

    #[test]
    fn case_sensitive_only_matches_exact() {
        let r = re("foo", false, true);
        let (out, n) = apply_rule("Foo FOO foo", &r, "x", false);
        assert_eq!(out, "Foo FOO x");
        assert_eq!(n, 1);
    }

    #[test]
    fn literal_mode_treats_special_chars_literally() {
        // "a.b" 在字面量模式下只匹配 "a.b"，不应像正则那样匹配 "axb"
        let r = re("a.b", false, true);
        let (out, n) = apply_rule("a.b axb", &r, "Z", false);
        assert_eq!(out, "Z axb");
        assert_eq!(n, 1);
    }

    #[test]
    fn literal_mode_keeps_dollar_in_replacement() {
        // 字面量模式下替换文本中的 "$1" 应原样写入，而不是当作捕获组
        let r = re("price", false, true);
        let (out, _) = apply_rule("price", &r, "$1.00", false);
        assert_eq!(out, "$1.00");
    }

    #[test]
    fn regex_mode_expands_capture_groups() {
        let r = re(r"(\d+)px", true, true);
        let (out, _) = apply_rule("12px", &r, "${1}rem", true);
        assert_eq!(out, "12rem");
    }

    #[test]
    fn analyze_reports_count_and_line_snippets() {
        let content = "line one foo\nno match here\nfoo and foo again";
        let r = re("foo", false, true);
        let (count, snippets) = analyze(content, &r, "BAR", false, 10);
        assert_eq!(count, 3);
        assert_eq!(snippets.len(), 2); // 两行有命中
        assert_eq!(snippets[0].line_no, 1);
        assert_eq!(snippets[0].after, "line one BAR");
        assert_eq!(snippets[1].line_no, 3);
        assert_eq!(snippets[1].after, "BAR and BAR again");
    }

    #[test]
    fn analyze_caps_snippets() {
        let content = "foo\nfoo\nfoo\nfoo";
        let r = re("foo", false, true);
        let (count, snippets) = analyze(content, &r, "x", false, 2);
        assert_eq!(count, 4);
        assert_eq!(snippets.len(), 2);
    }

    #[test]
    fn empty_find_is_rejected() {
        assert!(build_regex("", false, true).is_err());
    }

    #[test]
    fn invalid_regex_is_rejected() {
        assert!(build_regex("(unclosed", true, true).is_err());
    }
}
