use crate::{models::Language};

use super::WikiTextMacro;

mod test;

impl WikiTextMacro {
    pub fn find_first(language_section: &str, target_language: &Language) -> Option<Self> {
        let sections = language_section
            .split("\n===")
            .filter(
                |section| ["Noun===", "Verb===", "Adjective===", "Adverb==="].iter().any(|header|section.contains(header))
            )
            .collect::<Vec<&str>>();

        
        //todo Do we care to improve this and would it result in anythin
        let core_section = sections[0];


        // First try to find an {{infl and use that, ELSE look for language specific head macro
        
        if let Some(inflected_macro_start) = core_section.find(r#"{{infl"#) {
            let inflected_macro_end = core_section[inflected_macro_start..].find("}}").expect("closing brackets") + inflected_macro_start;

            return Some(WikiTextMacro { text: core_section[inflected_macro_start..inflected_macro_end + 2].to_string() })
        } else {
            let Some(open_brackets) = core_section.find({
                match target_language {
                    Language::Russian => {r#"{{ru-"#},
                    Language::Ukrainian => {r#"{{uk-"#},
                    Language::Belarusian => {r#"{{be-"#},
                }
            }) 
            else { 
                return None  // ! 
            }; 
            let Some(close_brackets) = core_section.find(r#"}}"#) else { return None } ;

            return Some(WikiTextMacro { text: core_section[open_brackets..close_brackets + 2].to_string() })
        };
    }


}