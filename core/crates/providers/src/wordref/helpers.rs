use scraper::{ElementRef, Node};

/// Returns the concatenated text of all direct `Node::Text` children,
/// trimming the provided characters from both ends of each fragment.
pub fn own_text(element: &ElementRef, trim: &[char]) -> String {
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
