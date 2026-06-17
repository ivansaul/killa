use domain::{PhoneticNotation, PronunciationAudio};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use std::sync::LazyLock;

use crate::wordref::helpers::text_content;

const AUDIO_BASE_URL: &str = "https://www.wordreference.com";

static PRONUNCIATION_WIDGET_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#pronunciation_widget").unwrap());

static PRON_WIDGET_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".pronWidget").unwrap());

static LISTEN_WIDGET_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#listen_widget").unwrap());

static AUDIO_FILES_SCRIPT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("script").unwrap());

static AUDIO_LABELS_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("#accentSelection optgroup[label=\"Accents\"] > option").unwrap()
});

pub fn parse_phonetics(html: &Html) -> Vec<PhoneticNotation> {
    let Some(widget) = html.select(&PRONUNCIATION_WIDGET_SELECTOR).next() else {
        return Vec::new();
    };

    widget
        .select(&PRON_WIDGET_SELECTOR)
        .map(|e| text_content(&e, &[' ', ':']))
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| PhoneticNotation {
            system: Some(String::from("ipa")),
            text: chunk[1].to_owned(),
            labels: vec![chunk[0].to_owned()],
        })
        .collect::<Vec<_>>()
}

pub fn parse_pronunciation_audios(html: &Html) -> Vec<PronunciationAudio> {
    let Some(root) = html.select(&LISTEN_WIDGET_SELECTOR).next() else {
        return Vec::new();
    };

    let audio_urls = extract_audio_urls(&root);
    let labels = extract_audio_labels(&root);

    audio_urls
        .iter()
        .zip(labels)
        .map(|(url, labels)| PronunciationAudio {
            url: normalize_audio_url(url),
            labels,
        })
        .collect()
}

fn extract_audio_urls(element: &ElementRef) -> Vec<String> {
    let Some(script) = element
        .select(&AUDIO_FILES_SCRIPT_SELECTOR)
        .map(|e| e.text().collect::<String>())
        .find(|s| s.contains("window.audioFiles"))
    else {
        return Vec::new();
    };

    // Matches URLs enclosed in single or double quotes or backticks
    let re = Regex::new(r#"['"`]([^'"`]+)['"`]"#).unwrap();
    re.captures_iter(&script)
        .map(|c| c[1].to_string())
        .collect::<Vec<_>>()
}

fn extract_audio_labels(element: &ElementRef) -> Vec<Vec<String>> {
    element
        .select(&AUDIO_LABELS_SELECTOR)
        .map(|e| {
            let mut labels = Vec::new();
            let text = e.text().collect::<String>();

            if !text.is_empty() {
                labels.push(text);
            }

            if let Some(title) = e.attr("title").filter(|s| !s.is_empty()) {
                labels.push(title.to_string());
            }

            labels
        })
        .collect::<Vec<_>>()
}

fn normalize_audio_url(path: &str) -> String {
    if path.starts_with("http") {
        path.to_string()
    } else {
        format!("{AUDIO_BASE_URL}{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_phonetics() {
        let fragment = r#"
            <div id="pronunciation_widget">
               <div class="pwrapper">
                  <input type="checkbox" class="more-pron-state" id="pronWgt">
                  <div class="more-pron-wrap">
                     <p class="pr"><span class="tooltip pronWidget">UK:<sup>*</sup><span style="font-size:12px"><sup>*</sup>UK and possibly other pronunciations</span></span><span class="pronWR tooltip pronWidget" dir="ltr"><span style="font-size:12px">UK and possibly other pronunciations</span>/praɪˈɒrɪti/</span></p>
                     <div class="more-pron-target">
                        <p class="pr"><span class="tooltip pronWidget">US:<span style="font-size:12px">USA pronunciation: IPA</span></span><span class="pronRH tooltip pronWidget" dir="ltr"><span style="font-size:12px">USA pronunciation: IPA</span>/praɪˈɔrɪti, -ˈɑr-/</span></p>
                        <p class="pr"><span class="tooltip pronWidget">US:<span style="font-size:12px">USA pronunciation: respelling</span></span><span class="pronRH tooltip pronWidget" dir="ltr"><span style="font-size:12px">USA pronunciation: respelling</span>(prī ôr<b>′</b>i tē, -or<b>′</b>-)</span></p>
                     </div>
                     <label for="pronWgt" class="more-pron-trigger"></label>
                  </div>
               </div>
            </div>
        "#;
        let html = Html::parse_fragment(&fragment);
        let phonetics = parse_phonetics(&html);
        assert_eq!(phonetics.len(), 3);
        assert_eq!(
            phonetics[0],
            PhoneticNotation {
                text: "/praɪˈɒrɪti/".into(),
                system: Some("ipa".into()),
                labels: vec!["UK".into()],
            }
        );
    }

    #[test]
    fn test_extract_audio_urls() {
        let fragment = r#"
            <div id="listen_widget">
                <script>window.audioFiles = ['/audio/es/Mexico/es052209.mp3','/audio/es/Castellano/es052209.mp3','/audio/es/Argentina/es052209.mp3',];</script>
            </div>
        "#;
        let html = Html::parse_fragment(&fragment);
        let element = html.select(&LISTEN_WIDGET_SELECTOR).next().unwrap();
        let urls = extract_audio_urls(&element);
        assert_eq!(
            urls,
            vec![
                "/audio/es/Mexico/es052209.mp3".to_string(),
                "/audio/es/Castellano/es052209.mp3".to_string(),
                "/audio/es/Argentina/es052209.mp3".to_string(),
            ]
        );
    }
}
