
use crate::{
    models::wiki_text_macro::WikiTextMacro, 
    constants, 
    RUN_OPTS
};
use std::io::Write;
use super::WikiTextPage;

mod test;

impl WikiTextPage {

    /// Target language's tags, based on RUN_OPTS selected language
    pub const INFLECTED_FORM_LABEL: &'static str = match RUN_OPTS.language {
        crate::models::Language::Russian => constants::page_labels::ru::INFLECTED_FORM,
        crate::models::Language::Ukrainian => constants::page_labels::ukr::INFLECTED_FORM,
        crate::models::Language::Belarusian => constants::page_labels::bela::INFLECTED_FORM,
    };


    pub fn write_lemma_entry(page: &str, diagnostics: bool, output: &mut impl Write) -> Result<(), String> {
        let (plain_word, lemma) = Self::extract_lemma_entry(page, diagnostics)?;
        
        output.write(format!("{plain_word},{lemma}\n").as_bytes()).expect("write");

        if plain_word.contains("ё") {
            output.write(format!("{plain_word_without_ё},{lemma}\n", plain_word_without_ё = plain_word.replace("ё", "е")).as_bytes(),).expect("write");
        }

        Ok(())
    }

    /// Attempts to extract (form, lemma) from page
    /// 
    /// 1. Find the plain word in question (page title, "form")
    /// 2. Find the target language section
    /// 3. Grab the first Noun, Verb, Adjective section, extracting the most immediate instance either of {{infl or {{ru- (for Russian)
    /// 4. Convert this {{macro}} into its contained lemma
    /// 5. Pair the form and lemma
    pub fn extract_lemma_entry(page: &str, diagnostics: bool) -> Result<(String, String), String>{

        let plain_word = Self::find_plain_word(&page).map_err(|e| format!("{e:#?}"))?; // This is the "form"
        if plain_word.chars().count() < 3 {
            return Err("Too short to be substantial".to_string());
        }

        let target_language_section = Self::find_language_section(&RUN_OPTS.language, page);
        let wiki_text_macro = WikiTextMacro::find_first(target_language_section, &RUN_OPTS.language);
        let lemma = wiki_text_macro.expect("Finding a macro in language section").extract_to_lemma();

        Ok((plain_word, lemma))
    }
}
