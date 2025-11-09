//! Build nested JSON output from document structure

use super::content::{parse_content, slugify};
use super::document::{Document, HeadingNode};
use super::output::*;
use std::path::Path;

/// Build complete JSON output with nested sections and markdown intelligence
pub fn build_json_output(doc: &Document, source_path: Option<&Path>) -> DocumentOutput {
    let tree = doc.build_tree();

    // Calculate metadata
    let max_depth = calculate_max_depth(&tree);
    let word_count = count_words(&doc.content);

    let metadata = DocumentMetadata {
        source: source_path.map(|p| p.to_string_lossy().to_string()),
        heading_count: doc.headings.len(),
        max_depth,
        word_count,
    };

    // Build sections with content
    let sections = tree
        .iter()
        .map(|node| build_section(node, &doc.content))
        .collect();

    DocumentOutput {
        document: DocumentRoot { metadata, sections },
    }
}

fn build_section(node: &HeadingNode, full_content: &str) -> Section {
    let heading = &node.heading;

    // Extract content for this section
    let (raw_content, offset, line) = extract_section_content(heading, full_content);

    // Parse content into blocks
    let blocks = parse_content(&raw_content, line);

    // Build child sections
    let children = node
        .children
        .iter()
        .map(|child| build_section(child, full_content))
        .collect();

    Section {
        id: slugify(&heading.text),
        level: heading.level,
        title: heading.text.clone(),
        slug: slugify(&heading.text),
        position: Position {
            line,
            offset,
        },
        content: Content {
            raw: raw_content,
            blocks,
        },
        children,
    }
}

fn extract_section_content(
    heading: &super::document::Heading,
    full_content: &str,
) -> (String, usize, usize) {
    // Find heading in content
    let search = format!("{} {}", "#".repeat(heading.level), heading.text);

    if let Some(offset) = full_content.find(&search) {
        // Calculate line number
        let line = full_content[..offset].lines().count() + 1;

        // Find end of section (next heading at same or higher level)
        let after_heading = &full_content[offset..];

        // Skip the heading line itself
        let content_start = after_heading.find('\n').map(|i| i + 1).unwrap_or(0);
        let section_content = &after_heading[content_start..];

        // Find next heading at same or higher level
        let end = find_next_heading(section_content, heading.level);

        (section_content[..end].trim().to_string(), offset + content_start, line + 1)
    } else {
        (String::new(), 0, 0)
    }
}

fn find_next_heading(content: &str, current_level: usize) -> usize {
    let mut in_code_block = false;
    let mut pos = 0;

    for line in content.lines() {
        // Track code block fences
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
        }

        // Check for heading only if not in code block
        if !in_code_block {
            if let Some(level) = get_heading_level(line) {
                if level <= current_level {
                    // Found next heading - return position
                    return pos;
                }
            }
        }

        pos += line.len() + 1; // +1 for newline
    }

    content.len()
}

fn get_heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let mut level = 0;

    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else if ch.is_whitespace() {
            return if level > 0 { Some(level) } else { None };
        } else {
            break;
        }
    }

    None
}

fn calculate_max_depth(tree: &[HeadingNode]) -> usize {
    tree.iter()
        .map(|node| 1 + calculate_max_depth(&node.children))
        .max()
        .unwrap_or(0)
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}
