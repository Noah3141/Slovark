use crate::models::pos::PoS;

use super::WikiTextPage;

impl WikiTextPage {
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