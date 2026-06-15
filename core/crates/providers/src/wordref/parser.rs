use domain::{Example, PartOfSpeech, Sense, Translation};
use scraper::{ElementRef, Node, Selector};

use crate::wordref::{
    models::{ParsedRow, SenseStartRow, TranslationRow},
    selectors::{
        FR_EX_SELECTOR, FR_WRD_SELECTOR, POS_SELECTOR, ROW_SELECTOR, STRONG_SELECTOR, TD_SELECTOR,
        TO_EX_SELECTOR, TO_WRD_SELECTOR,
    },
};

pub fn parse_table(table: &ElementRef) -> Vec<Sense> {
    let rows = table.select(&ROW_SELECTOR);

    let mut senses = Vec::new();
    let mut current_sense: Option<Sense> = None;
    let mut pending_example_sources: Vec<String> = Vec::new();

    for row in rows {
        match parse_row(&row) {
            ParsedRow::SenseStart(data) => {
                if let Some(sense) = current_sense.take() {
                    senses.push(sense);
                }

                current_sense = Some(Sense {
                    part_of_speech: data.part_of_speech,
                    source_term: data.source_term,
                    gloss: data.gloss,
                    translations: data.translations,
                    examples: Vec::new(),
                    definitions: Vec::new(),
                });
            }

            ParsedRow::Translation(data) => {
                if let Some(sense) = current_sense.as_mut() {
                    sense.translations.extend(data.translations);

                    if sense.gloss.is_none() {
                        sense.gloss = data.gloss;
                    }
                }
            }

            ParsedRow::ExampleSource(examples) => {
                pending_example_sources = examples;
            }

            ParsedRow::ExampleTarget(targets) => {
                if let Some(sense) = current_sense.as_mut() {
                    for (source, target) in
                        pending_example_sources.drain(..).zip(targets.into_iter())
                    {
                        sense.examples.push(Example {
                            source,
                            target: Some(target),
                        });
                    }
                }
            }

            ParsedRow::Unknown => {}
        }
    }

    if let Some(sense) = current_sense {
        senses.push(sense);
    }

    senses
}

fn parse_row(row: &ElementRef) -> ParsedRow {
    let is_new_sense = row.value().attr("id").is_some();

    if is_new_sense {
        return ParsedRow::SenseStart(parse_sense_start(row));
    }

    if row.select(&FR_EX_SELECTOR).next().is_some() {
        return ParsedRow::ExampleSource(parse_examples(row, &FR_EX_SELECTOR));
    }

    if row.select(&TO_EX_SELECTOR).next().is_some() {
        return ParsedRow::ExampleTarget(parse_examples(row, &TO_EX_SELECTOR));
    }

    if row.select(&TO_WRD_SELECTOR).next().is_some() {
        return ParsedRow::Translation(parse_translation_row(row));
    }

    ParsedRow::Unknown
}

fn parse_sense_start(row: &ElementRef) -> SenseStartRow {
    SenseStartRow {
        source_term: parse_source_term(row),
        part_of_speech: parse_sense_pos(row),
        gloss: parse_gloss(row),
        translations: parse_translations(row),
    }
}

fn parse_translation_row(row: &ElementRef) -> TranslationRow {
    TranslationRow {
        gloss: parse_gloss(row),
        translations: parse_translations(row),
    }
}

fn parse_source_term(row: &ElementRef) -> String {
    row.select(&FR_WRD_SELECTOR)
        .next()
        .and_then(|fr| fr.select(&STRONG_SELECTOR).next())
        .map(|e| text_content(&e))
        .unwrap_or_default()
}

fn parse_gloss(row: &ElementRef) -> Option<String> {
    row.select(&TD_SELECTOR)
        .nth(1)
        .map(|e| clean_text(&e))
        .map(|s| s.trim_matches(['(', ')', ' ']).to_string())
        .filter(|s| !s.is_empty())
}

fn parse_translations(row: &ElementRef) -> Vec<Translation> {
    let Some(to_wrd) = row.select(&TO_WRD_SELECTOR).next() else {
        return Vec::new();
    };

    let pos = parse_translation_pos(&to_wrd);
    to_wrd
        .children()
        .filter_map(|node| match node.value() {
            Node::Text(text) => Some(text),
            _ => None,
        })
        .map(|s| s.replace(",", "").trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|text| Translation {
            text: text.to_string(),
            part_of_speech: pos.clone(),
        })
        .collect::<Vec<_>>()
}

fn parse_examples(row: &ElementRef, selector: &Selector) -> Vec<String> {
    row.select(selector)
        .next()
        .map(|el| clean_text(&el))
        .map(|text| {
            text.split("//")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

fn parse_sense_pos(row: &ElementRef) -> Option<PartOfSpeech> {
    row.select(&FR_WRD_SELECTOR)
        .next()
        .and_then(|el| parse_pos_from_element(&el))
}

fn parse_translation_pos(element: &ElementRef) -> Option<PartOfSpeech> {
    parse_pos_from_element(element)
}

fn parse_pos_from_element(element: &ElementRef) -> Option<PartOfSpeech> {
    let abbr = element
        .select(&POS_SELECTOR)
        .next()?
        .value()
        .attr("data-abbr")?;

    Some(match abbr {
        "n" | "nm" | "nf" => PartOfSpeech::Noun,
        "adj" => PartOfSpeech::Adjective,
        "adv" => PartOfSpeech::Adverb,
        "vi" | "vtr" | "v" => PartOfSpeech::Verb,
        "prep" => PartOfSpeech::Preposition,
        "interj" => PartOfSpeech::Interjection,
        "loc verb" => PartOfSpeech::Phrase,
        "expr" => PartOfSpeech::Expression,
        other => PartOfSpeech::Unknown(other.into()),
    })
}

/// Returns the concatenated text content of all `Node::Text` within this element
fn text_content(element: &ElementRef) -> String {
    element
        .children()
        .filter_map(|node| match node.value() {
            Node::Text(text) => Some(text),
            _ => None,
        })
        .map(|s| s.trim_matches([' ', '"']))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn clean_text(element: &ElementRef) -> String {
    element
        .text()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use crate::wordref::{parser::parse_source_term, selectors::ROW_SELECTOR};

    #[test]
    fn test_parse_source_term() {
        let html = r#"
        <table>
            <tr class="odd" id="enes:2151:1">
                <td class="FrWrd">
                    <strong>"run " <span title="something">[sth]</span></strong>
                    <a title="conjugate run" class="conjugate" href="/conj/enverbs.aspx?v=run">⇒</a>
                    <em class="POS2" data-lang="en" data-abbr="vi">vi</em>
                </td>
            </tr>
        </table>
        "#;

        let fragment = Html::parse_fragment(html);
        let row = fragment.select(&ROW_SELECTOR).next().unwrap();

        let source_term = parse_source_term(&row);
        assert_eq!(source_term, "run");
    }
}
