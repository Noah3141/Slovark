use crate::RUN_OPTS;
use super::{InitError, WikiTextPage};

mod find_page_id;
mod find_plain_word;
mod find_language_section;
mod determine_pos;
mod find_accented_word;
mod find_related_words;
mod find_table;
mod find_etymology;


impl WikiTextPage {

    const TITLE_START: &'static str = "<title>";
    const TITLE_END: &'static str = "</title>";

    /// Return a `Self` if the block expresses a head (infinitive, dictionary form noun, etc.)
    /// Else return a string containing the oblique plain_word in question
    pub fn parse_from_page(page: &str, diagnostics: bool) -> Result<Self, InitError> {
        let page_id = Self::find_id(&page[..])?; // 1
        let plain_word = Self::find_plain_word(&page[..])?; // 2
        if plain_word.chars().count() < 3 {
            return Err(InitError::NotASubstantiveWord(
                "Few letter word".to_string(),
            ));
        }

        let target_language_section = Self::find_language_section(&RUN_OPTS.language, page);

        // Self::extract_lemma_entry(&russian_section, &plain_word);

        if diagnostics {
            println!("\x1b[31m&&&&&&&&&start\n{target_language_section}&&&&&&&&end\n\x1b[0m");
        }

        let pos = Self::determine_pos(target_language_section)?; // 3
        let accented_word =
            Self::find_accented_word(target_language_section)?.unwrap_or(plain_word.clone()); // 4
        let table = Self::find_table(target_language_section, &pos, &plain_word)?; // 5
        let etymology = Self::find_etymology(target_language_section)?; // 6
        let related_terms = Self::find_related_terms(target_language_section)?; // 7

        Ok(Self {
            plain_word,
            accented_word,
            etymology,
            page_id,
            pos,
            related_terms,
            table,
        })
    }


    pub fn pronunciation(self) -> String {
        format!("{{{{ru-IPA|{}}}}}", self.accented_word)
    }
}