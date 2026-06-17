use scraper::{ElementRef, Node};

/// Returns the concatenated text of all direct `Node::Text` children,
/// trimming the provided characters from both ends of each fragment.
///
/// Example:
///
/// ```rust
/// let html = r#"<td>("cover a distance")</td>"#;
/// let fragment = scraper::Html::parse_fragment(html);
/// let selector = scraper::Selector::parse("td").unwrap();
/// let element = fragment.select(&selector).next().unwrap();
/// assert_eq!(own_text(&element, &['"', ' ', '(', ')']), "cover a distance".to_string());
/// ```
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

/// Returns the concatenated text of all descendant text nodes
/// trimming whitespace from each fragment and joining them with a single space.
///
/// Example:
///
/// ```rust
/// let html = r#"<strong>run <span title="something"> [sth]</span></strong>"#;
/// let fragment = scraper::Html::parse_fragment(html);
/// let selector = scraper::Selector::parse("strong").unwrap();
/// let element = fragment.select(&selector).next().unwrap();
/// assert_eq!(full_text(&element), "run [sth]".to_string());
/// ```
pub fn full_text(element: &ElementRef) -> String {
    element
        .text()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
