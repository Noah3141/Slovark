
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
    /// 1. Find the plain word in question (page title)
    /// 2. Find the target language section
    /// 3. Check if the target language section includes the special labels for inflected form (indicating, say, that кошки should link to кошка) or non-inflected form (detected by )
    /// 4.  
    pub fn extract_lemma_entry(page: &str, diagnostics: bool) -> Result<(String, String), String>{

        let plain_word = Self::find_plain_word(&page).map_err(|e| format!("{e:#?}"))?; // 2
        if plain_word.chars().count() < 3 {
            return Err("Too short to be substantial".to_string());
        }

        let target_language_section = Self::find_language_section(&RUN_OPTS.language, page);
        //
        // ==RU/BE/UK== 
        // ===Verb===
        //
        // ===Noun===
        // 

        let wiki_text_macro = WikiTextMacro::find_first(target_language_section, &RUN_OPTS.language);
        let lemma = wiki_text_macro.expect("Finding a macro in language section").extract_to_lemma();

        Ok((plain_word, lemma))

        // if target_language_section.contains(Self::INFLECTED_FORM_LABEL) {
        //     let start_of_lemma = target_language_section.find(Self::INFLECTED_FORM_LABEL).unwrap()
        //         + Self::INFLECTED_FORM_LABEL.len();
        //     let lemma_onward = &target_language_section[start_of_lemma..];
        //     let mut lemma = &lemma_onward[..lemma_onward.find("||").expect("closing pipes")];

        //     if lemma.contains("|") {
        //         lemma = &lemma[..lemma.find("|").unwrap()];
        //     }

        //     return Ok((plain_word, lemma.to_string()))


        // } else if target_language_section.contains(Self::NONINFLECTED_FORM_LABEL) {
        //     let noun_macro = NounMacro::find_first_in(target_language_section).expect("macro to be found");
        //     let lemma = match noun_macro.core_noun_text() {
        //         Some(text) => text,
        //         None => {
        //             return Err("Couldn't locate core noun text in noun macro".to_string());
        //         }
        //     };

        //     return Ok((plain_word, lemma))
            
        // }


    }
}
