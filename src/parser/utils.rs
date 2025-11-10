//! Utility functions for markdown parsing.
//!
//! Shared helper functions used across the parser module.

/// Strip inline markdown formatting (bold, italic, code, strikethrough) from text.
///
/// This is useful when comparing heading text extracted from events (which strips formatting)
/// against the raw markdown source (which contains formatting).
///
/// # Examples
///
/// ```
/// # use treemd::parser::utils::strip_markdown_inline;
/// assert_eq!(strip_markdown_inline("**bold** text"), "bold text");
/// assert_eq!(strip_markdown_inline("`code` here"), "code here");
/// ```
pub fn strip_markdown_inline(text: &str) -> String {
    text.replace("**", "")
        .replace("*", "")
        .replace("`", "")
        .replace("~~", "")
}

/// Extract the heading level from a line of markdown text.
///
/// Returns `Some(level)` if the line is a valid heading (1-6 #'s followed by whitespace),
/// or `None` otherwise.
///
/// # Examples
///
/// ```
/// # use treemd::parser::utils::get_heading_level;
/// assert_eq!(get_heading_level("# Title"), Some(1));
/// assert_eq!(get_heading_level("## Section"), Some(2));
/// assert_eq!(get_heading_level("not a heading"), None);
/// assert_eq!(get_heading_level("#NoSpace"), None);
/// ```
pub fn get_heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let mut level = 0;

    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else if ch.is_whitespace() {
            return if level > 0 && level <= 6 {
                Some(level)
            } else {
                None
            };
        } else {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_markdown_inline() {
        assert_eq!(strip_markdown_inline("**bold**"), "bold");
        assert_eq!(strip_markdown_inline("*italic*"), "italic");
        assert_eq!(strip_markdown_inline("`code`"), "code");
        assert_eq!(strip_markdown_inline("~~strike~~"), "strike");
        assert_eq!(
            strip_markdown_inline("**turbocli-parser** (850 LOC)"),
            "turbocli-parser (850 LOC)"
        );
    }

    #[test]
    fn test_get_heading_level() {
        assert_eq!(get_heading_level("# Title"), Some(1));
        assert_eq!(get_heading_level("## Section"), Some(2));
        assert_eq!(get_heading_level("### Subsection"), Some(3));
        assert_eq!(get_heading_level("#### Level 4"), Some(4));
        assert_eq!(get_heading_level("##### Level 5"), Some(5));
        assert_eq!(get_heading_level("###### Level 6"), Some(6));

        // Invalid cases
        assert_eq!(get_heading_level("not a heading"), None);
        assert_eq!(get_heading_level("#NoSpace"), None);
        assert_eq!(get_heading_level("####### Too many"), None);
        assert_eq!(get_heading_level("  ## Indented"), Some(2)); // Trimmed
    }
}
