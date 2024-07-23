use std::{fmt::Display, fs::File, io::Write};

use serde::{Deserialize, Serialize};

use crate::models::wiki_text_macro::NounMacro;

use super::{pos::PoS, Language};

#[derive(Debug, Serialize, Deserialize)]
/// Faithful parsing of the WikiText strings in a page. Holds verbs, nouns, and adjective entries alike
pub struct WikiTextPage {
    pub page_id: u64,
    pub pos: PoS,
    pub plain_word: String,
    pub accented_word: String,
    pub table: String,
    pub etymology: Option<String>,
    pub related_terms: Option<String>,
}

#[derive(Debug)]
pub enum WikiTextPageInitError {
    MissingCorePiece(String),
    MissingExtraPiece(String),
    NotADictionaryPage(String),
    UnimplementedPOSFound(String),
    NotASubstantiveWord(String),
    InflectedFormPage,
    UndeclinableNoun,
}

type InitError = WikiTextPageInitError;

impl WikiTextPage {
    const ID_START: &'static str = "<id>";
    const ID_END: &'static str = "</id>";
    const TITLE_START: &'static str = "<title>";
    const TITLE_END: &'static str = "</title>";

    fn test() {
        Self::find_accented_word(r#"""=======================================================start

<page>
    <title>полседьмого</title>
    <ns>0</ns>
    <id>5036007</id>
    <revision>
      <id>66991125</id>
      <parentid>51816945</parentid>
      <timestamp>2022-05-15T09:22:07Z</timestamp>
      <contributor>
        <username>WingerBot</username>
        <id>2024159</id>
      </contributor>
      <minor />
      <comment>use {{af}} in place of {{prefix}}/{{suffix}}/{{affix}} etc.; move {{wikipedia}} lines to top of etym section (automatic + manually assisted)</comment>
      <origin>66991125</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="512" sha1="k4unqvyy96azadadhxm08yjloz8lrpk" xml:space="preserve">==Russian==

===Etymology===
{{af|ru|пол-|седьмо́й|t1=[[half]]|t2=[[seventh]]}}, with {{m|ru|седьмо́й}} in the genitive case, as with other words prefixed with {{m|ru|пол-}}. Compare {{cog|de|halb sieben||half past six|lit=half seven}}.

===Pronunciation===
* {{ru-IPA|по̀лседьмо́го}}

===Noun===
{{ru-noun|полседьмо́го|tr=polsedʹmóvo|n-in|-}}

# {{lb|ru|colloquial}} [[half past]] [[six]], [[six-thirty]] {{gloss|hour of the day}}

====Declension====
Invariable.</text>
      <sha1>k4unqvyy96azadadhxm08yjloz8lrpk</sha1>
    </revision>
  </page>

=======================================================end"""#).expect("to find it");
    }

    /// Return a `Self` if the block expresses a head (infinitive, dictionary form noun, etc.)
    /// Else return a string containing the oblique plain_word in question
    pub fn parse_russian(page: &str, diagnostics: bool) -> Result<Self, InitError> {
        let page_id = Self::find_id(&page[..])?; // 1
        let plain_word = Self::find_plain_word(&page[..])?; // 2
        if plain_word.chars().count() < 3 {
            return Err(InitError::NotASubstantiveWord(
                "Few letter word".to_string(),
            ));
        }

        let russian_section = Self::find_language_section(Language::Russian, page);

        // Self::extract_lemma_entry(&russian_section, &plain_word);

        if diagnostics {
            println!("\x1b[31m&&&&&&&&&start\n{russian_section}&&&&&&&&end\n\x1b[0m");
        }

        let pos = Self::determine_pos(russian_section)?; // 3
        let accented_word =
            Self::find_accented_word(russian_section)?.unwrap_or(plain_word.clone()); // 4
        let table = Self::find_table(russian_section, &pos, &plain_word)?; // 5
        let etymology = Self::find_etymology(russian_section)?; // 6
        let related_terms = Self::find_related_terms(russian_section)?; // 7

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

    /// 1
    fn find_id(page: &str) -> Result<u64, InitError> {
        let start: usize = page
            .find(Self::ID_START)
            .ok_or(InitError::MissingCorePiece(
                "No start id tag found.".to_string(),
            ))?
            + Self::ID_START.len();
        let end: usize = page.find(Self::ID_END).ok_or(InitError::MissingCorePiece(
            "No end id tag found.".to_string(),
        ))?;
        let id: u64 = page[start..end]
            .parse::<u64>()
            .map_err(|_| InitError::MissingCorePiece("Failed to parse as int.".to_string()))?;
        Ok(id)
    }

    /// 2
    fn find_plain_word(page: &str) -> Result<String, InitError> {
        let start = page
            .find(Self::TITLE_START)
            .ok_or(InitError::MissingCorePiece(
                "No start title tag found.".to_string(),
            ))?
            + Self::TITLE_START.len();
        let end = page
            .find(Self::TITLE_END)
            .ok_or(InitError::MissingCorePiece(
                "No end title tag found.".to_string(),
            ))?;
        let title = page[start..end].to_string();

        if title.starts_with("Wiktionary:") || title.starts_with("User:") {
            return Err(InitError::NotADictionaryPage(page.to_string()));
        }
        Ok(title)
    }

    /// 3
    pub fn find_language_section(language: Language, page: &str) -> &str {
        // Now parse down the Russian section only via detecting any OTHER sections
        match language {
            Language::Russian => {
                let russian_start = page.find("==Russian==").expect("No russian section found!");
                let russian_onward = &page[russian_start..];
                let section_headers = [
                    "==Ukrainian==",
                    "==Serbo-Croatian==",
                    "==Tabasaran==",
                    "==Udmurt==",
                    "==Southern Altai==",
                    "==Tatar==",
                    "==Tundra Nenets==",
                    "==Tuvan==",
                    "==Wakhi==",
                    "==Western Mari==",
                    "==Ubykh==",
                    "==Yakut==",
                ];

                let any_section_switch = section_headers
                    .iter()
                    .filter_map(|&header| russian_onward.find(header).map(|i| i + russian_start))
                    .min()
                    .unwrap_or(page.len());
                let russian_section = &page[russian_start..any_section_switch];

                russian_section
            }
            Language::Ukrainian => {
                panic!("Not implemented")
            }
            Language::Belarusian => {
                panic!("Not implemented")
            }
        }
    }

    /// 4
    fn determine_pos(russian_section: &str) -> Result<PoS, InitError> {
        if let Some(_) = russian_section.find("===Noun===") {
            return Ok(PoS::Noun);
        };
        if let Some(_) = russian_section.find("===Verb===") {
            return Ok(PoS::Verb);
        };
        if let Some(_) = russian_section.find("===Adjective===") {
            return Ok(PoS::Adjective);
        };
        if let Some(_) = russian_section.find("===Participle===") {
            return Ok(PoS::Adjective);
        }
        if let Some(_) = russian_section.find("===Adverb===") {
            return Ok(PoS::Adverb);
        };
        if let Some(_) = russian_section.find("===Predicative===") {
            return Ok(PoS::Adverb);
        };
        if let Some(_) = russian_section.find("===Numeral===") {
            return Ok(PoS::Noun);
        };

        // if let Some(_) = russian_section.find("===Proper noun===") { return Err(InitError::NotASubstantiveWord("Proper noun which is niche for learners".to_string()))}
        // if let Some(_) = russian_section.find("===Prepositional phrase===") { return Err(InitError::NotASubstantiveWord("Phrases shan't be parsed as vocabulary terms".to_string()))}
        // if let Some(_) = russian_section.find("===Phrase===") { return Err(InitError::NotASubstantiveWord("Phrases shan't be parsed as vocabulary terms".to_string()))}
        // if let Some(_) = russian_section.find("===Proverb===") { return Err(InitError::NotASubstantiveWord("Phrases shan't be parsed as vocabulary terms".to_string()))}
        // if let Some(_) = russian_section.find("===Preposition===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Idiom===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Conjunction===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Combining form===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Suffix===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Interfix===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Prefix===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Determiner===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Pronoun===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Particle===") { return Err(InitError::NotASubstantiveWord("Figure it out dude".to_string()))}
        // if let Some(_) = russian_section.find("===Interjection===") { return Err(InitError::NotASubstantiveWord("We'll just link to Wiktionary for these they don't need dictionary_info".to_string()))}

        Err(InitError::NotASubstantiveWord(String::from("Catchall")))

        // Err(InitError::UnimplementedPOSFound(String::from("No POS determinable! Probably an adverb or somethin, better get on that")))
    }

    /// 5 Finds the accented representation of the word, e.g. ко́шка. These are absent when single syllable, where the Russian spelling is proxied by just having the wikitext say "{{ru-IPA}}". In these cases, return None, and we'll set the accented = plain
    fn find_accented_word(russian_section: &str) -> Result<Option<String>, InitError> {
        // {{ru-IPA|эска́дренный бронено́сец}}
        let pronunciation_start: usize =
            russian_section
                .find("===Pronunciation===")
                .ok_or(InitError::MissingExtraPiece(
                    "No pronunciation section!".to_string(),
                ))?;
        let pronunciation: &str = &russian_section[pronunciation_start..];
        let start: usize = pronunciation.find("{{").ok_or(InitError::MissingCorePiece(
            "Open bracket to pronunciation missing!".to_string(),
        ))?;
        let end: usize = pronunciation.find("}}").ok_or(InitError::MissingCorePiece(
            "Closing bracket to pronunciation missing!".to_string(),
        ))?;

        let bracketed_word = pronunciation[start..end].to_string();
        let word = if bracketed_word.starts_with("{{ru-IPA|") {
            Some(bracketed_word["{{ru-IPA|".len()..].to_string())
        } else {
            return Ok(None);
        };
        Ok(word)
    }

    /// 6
    fn find_table(russian_section: &str, pos: &PoS, plain_word: &str) -> Result<String, InitError> {
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

        static mut WRT_PTR: usize = 0;
        static mut LAST_WRT_LEN: usize = 0;

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

    /// 7
    fn find_etymology(russian_section: &str) -> Result<Option<String>, InitError> {
        let etymology_start = russian_section.find("===Etymology===");
        match etymology_start {
            None => Ok(None),
            Some(start) => {
                let etymology: &str = &russian_section[start..];
                let start: usize = etymology.find("{{").ok_or(InitError::MissingExtraPiece(
                    "Open bracket to etymology missing!".to_string(),
                ))?;
                let end: usize = etymology.find("}}").ok_or(InitError::MissingExtraPiece(
                    "Closing bracket to etymology missing!".to_string(),
                ))?;
                Ok(Some(etymology[start..end + 2].to_string()))
            }
        }
    }

    /// 8
    fn find_related_terms(russian_section: &str) -> Result<Option<String>, InitError> {
        let related_terms_start = match russian_section.find("====Related terms====") {
            Some(loc) => loc,
            None => return Ok(None),
        };
        let related_terms: &str = &russian_section[related_terms_start..];
        let start: usize = related_terms
            .find("{{")
            .ok_or(InitError::MissingExtraPiece(
                "Open bracket to related terms missing!".to_string(),
            ))?;
        let end: usize = related_terms
            .find("}}")
            .ok_or(InitError::MissingExtraPiece(
                "Closing bracket to related terms missing!".to_string(),
            ))?;
        Ok(Some(related_terms[start..end + 2].to_string()))
    }

    pub fn pronunciation(self) -> String {
        format!("{{{{ru-IPA|{}}}}}", self.accented_word)
    }

    const INFLECTED_FORM_LABEL: &'static str = "inflection of|ru|";
    const NONINFLECTED_FORM_LABEL: &'static str = "{{ru-noun+|";

    fn extract_lemma_entry(russian_section: &str, plain_word: &str, mut output: impl Write) {
        if russian_section.contains(Self::INFLECTED_FORM_LABEL) {
            let start_of_lemma = russian_section.find(Self::INFLECTED_FORM_LABEL).unwrap()
                + Self::INFLECTED_FORM_LABEL.len();
            let lemma_onward = &russian_section[start_of_lemma..];
            let mut lemma = &lemma_onward[..lemma_onward.find("||").expect("closing pipes")];

            if lemma.contains("|") {
                lemma = &lemma[..lemma.find("|").unwrap()];
            }

            // println!("@ {plain_word},{lemma}");
            output
                .write(format!("{plain_word},{lemma}\n").as_bytes())
                .expect("write");

            if plain_word.contains("ё") {
                output
                    .write(
                        format!(
                            "{plain_word_without_ё},{lemma}\n",
                            plain_word_without_ё = plain_word.replace("ё", "е")
                        )
                        .as_bytes(),
                    )
                    .expect("write");
            }
        } else if russian_section.contains(Self::NONINFLECTED_FORM_LABEL) {
            let noun_macro = NounMacro::find_first_in(russian_section).expect("macro to be found");
            let lemma = match noun_macro.core_noun_text() {
                Some(text) => text,
                None => {
                    return;
                }
            };

            output
                .write(format!("{plain_word},{lemma}\n").as_bytes())
                .expect("write");

            if plain_word.contains("ё") {
                output
                    .write(
                        format!(
                            "{plain_word_without_ё},{lemma}\n",
                            plain_word_without_ё = plain_word.replace("ё", "е")
                        )
                        .as_bytes(),
                    )
                    .expect("write");
            }
        } else {
        }
    }

    //
    //
    //

    pub fn get_full_info(self) -> rubit_api_db::models::russian_entry::Model {
        match self.pos {
            PoS::Noun => self.get_noun_info(),
            PoS::Verb => self.get_verb_info(),
            PoS::Adjective => self.get_adjective_info(),
            PoS::Adverb => self.get_adverb_info(),
        }
    }

    fn get_noun_info(self) -> rubit_api_db::models::russian_entry::Model {
        rubit_api_db::models::russian_entry::Model {
            id: todo!(),
            lemma: todo!(),
            commonality: todo!(),
            r#type: todo!(),
            dictionary_info: todo!(),
        }
    }
    fn get_verb_info(self) -> rubit_api_db::models::russian_entry::Model {
        rubit_api_db::models::russian_entry::Model {
            id: todo!(),
            lemma: todo!(),
            commonality: todo!(),
            r#type: todo!(),
            dictionary_info: todo!(),
        }
    }
    fn get_adjective_info(self) -> rubit_api_db::models::russian_entry::Model {
        rubit_api_db::models::russian_entry::Model {
            id: todo!(),
            lemma: todo!(),
            commonality: todo!(),
            r#type: todo!(),
            dictionary_info: todo!(),
        }
    }
    fn get_adverb_info(self) -> rubit_api_db::models::russian_entry::Model {
        rubit_api_db::models::russian_entry::Model {
            id: todo!(),
            lemma: todo!(),
            commonality: todo!(),
            r#type: todo!(),
            dictionary_info: todo!(),
        }
    }
}
