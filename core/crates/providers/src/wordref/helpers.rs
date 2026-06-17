use scraper::{ElementRef, Node};

/// Returns the concatenated text content of all `Node::Text` children,
/// trimming the provided characters from both ends of each fragment.
pub fn text_content(element: &ElementRef, trim: &[char]) -> String {
    element
        .children()
        .filter_map(|node| match node.value() {
            Node::Text(text) => Some(text),
            _ => None,
        })
        .map(|s| s.trim_matches(trim))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
