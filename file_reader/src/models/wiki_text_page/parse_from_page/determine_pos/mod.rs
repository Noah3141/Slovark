mod test;
use crate::models::pos::PoS;

use super::{InitError, WikiTextPage};

impl WikiTextPage {
    /// 4
    pub fn determine_pos(russian_section: &str) -> Result<PoS, InitError> {
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
}