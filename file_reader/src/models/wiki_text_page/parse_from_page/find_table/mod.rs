use crate::models::{pos::PoS, wiki_text_page::error::WikiTextPageInitError, };
mod test;

use super::{InitError, WikiTextPage};

impl WikiTextPage {
    /// 6
    pub fn find_table(russian_section: &str, pos: &PoS, plain_word: &str) -> Result<String, InitError> {
        let result = match pos {
            PoS::Noun => {
                russian_section
                    .find("====Declension====")
                    .ok_or(InitError::MissingExtraPiece(
                        "No declension section for noun!".to_string(),
                    ))
            }
            PoS::Verb => {
                russian_section
                    .find("====Conjugation====")
                    .ok_or(InitError::MissingExtraPiece(
                        "No conjugation section for verb!".to_string(),
                    ))
            }
            PoS::Adjective => {
                russian_section
                    .find("====Declension====")
                    .ok_or(InitError::MissingExtraPiece(
                        "No declension section for adj!".to_string(),
                    ))
            }
            PoS::Adverb => return Ok(String::from("Adverbs do not inflect")),
        };


        match result {
            Err(e) => {
                if russian_section.contains(Self::INFLECTED_FORM_LABEL) {
                    return Err(InitError::InflectedFormPage);
                } else {
                    return Err(e);
                }
            }
            Ok(table_start) => {
                let subsection: &str = &russian_section[table_start..];
                let start: usize = subsection.find("{{").ok_or({
                    if subsection.contains("Invariable") {
                        InitError::UndeclinableNoun
                    } else {
                        InitError::MissingCorePiece(format!(
                            "Open bracket to table missing!\n{subsection}\n\n"
                        ))
                    }
                })?;
                let end: usize = subsection.find("}}").ok_or(InitError::MissingCorePiece(
                    "Closing bracket to table missing!".to_string(),
                ))?;

                let attempted_table = subsection[start..end + 2].to_string();

                if attempted_table.contains("...quot;") {
                    return Err(WikiTextPageInitError::MissingCorePiece(
                        "Something is wrong with parsing".to_string(),
                    ));
                }
                Ok(attempted_table)
            }
        }
    }
}