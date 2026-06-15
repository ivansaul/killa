#[derive(Debug)]
pub enum LanguageCode {
    En,
    Es,
}

impl LanguageCode {
    pub fn as_str(&self) -> &str {
        match self {
            LanguageCode::En => "en",
            LanguageCode::Es => "es",
        }
    }
}

#[derive(Debug, Clone)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Phrase,
    Expression,
    Unknown(String),
}

/// Represents a sense of a word
#[derive(Debug, Default)]
pub struct Sense {
    /// The part of speech of the sense.
    ///
    /// e.g. `noun`, `verb`, `adjective`
    pub part_of_speech: Option<PartOfSpeech>,
    /// The source term of the sense.
    pub source_term: String,
    /// Short contextual hint used to disambiguate meanings.
    pub gloss: Option<String>,
    /// Available translations for this sense.
    pub translations: Vec<Translation>,
    /// Real-world usage examples.
    pub examples: Vec<Example>,
    /// Detailed definitions of the sense.
    pub definitions: Vec<Definition>,
}

#[derive(Debug)]
pub struct Definition {
    pub text: String,
    pub examples: Vec<Example>,
}

#[derive(Debug)]
pub struct Translation {
    pub text: String,
    pub part_of_speech: Option<PartOfSpeech>,
}

/// Represents an example sentence or phrase with its source and translation.
#[derive(Debug)]
pub struct Example {
    /// The source sentence.
    pub source: String,
    /// The translated sentence, if available.
    pub target: Option<String>,
}
