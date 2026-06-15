use domain::{PartOfSpeech, Translation};

#[derive(Debug)]
pub enum ParsedRow {
    SenseStart(SenseStartRow),
    Translation(TranslationRow),
    ExampleSource(Vec<String>),
    ExampleTarget(Vec<String>),
    Unknown,
}

#[derive(Debug)]
pub struct SenseStartRow {
    pub source_term: String,
    pub part_of_speech: Option<PartOfSpeech>,
    pub gloss: Option<String>,
    pub translations: Vec<Translation>,
}

#[derive(Debug)]
pub struct TranslationRow {
    pub gloss: Option<String>,
    pub translations: Vec<Translation>,
}
