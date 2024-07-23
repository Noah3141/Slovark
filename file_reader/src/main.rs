#[allow(unused, unused_variables, dead_code)]
use std::fs::File;
mod processes;
use models::Language;

mod constants;
mod models;
mod traits;

// Settings

pub struct RunOpts {
    pub language: Language,
    pub id: u16,
    pub input_wiki_dump: &'static str,
}
impl RunOpts {
    pub fn increment_id(mut self) {
        self.id += 1;
    }

    pub fn intermediary_out(&self) -> String {
        match self.language {
            crate::models::Language::Russian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ru\\2. intermediary\\{id}_ru_intermediary.txt", id= RUN_OPTS.id)
            }
            crate::models::Language::Ukrainian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ukr\\2. intermediary\\{id}_ukr_intermediary.txt", id= RUN_OPTS.id)
            }
            crate::models::Language::Belarusian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_bela\\2. intermediary\\{id}_bela_intermediary.txt", id= RUN_OPTS.id)
            }
        }
    }

    pub fn wiki_pages_out(&self) -> String {
        match self.language {
            crate::models::Language::Russian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ru\\3. wikipage_json\\{id}_ru_wiki_pages.json", id= RUN_OPTS.id)
            }
            crate::models::Language::Ukrainian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ukr\\3. wikipage_json\\{id}_ukr_wiki_pages.json", id= RUN_OPTS.id)
            }
            crate::models::Language::Belarusian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_bela\\3. wikipage_json\\{id}_bela_wiki_pages.json", id= RUN_OPTS.id)
            }
        }
    }
    pub fn database_entries_out(&self) -> String {
        match self.language {
            crate::models::Language::Russian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ru\\4. database_entries\\{id}_ru_entries.csv", id= RUN_OPTS.id)
            }
            crate::models::Language::Ukrainian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_ukr\\4. database_entries\\{id}_ukr_entries.csv", id= RUN_OPTS.id)
            }
            crate::models::Language::Belarusian => {
                format!("C:\\Users\\Noah3\\Code\\Slovark\\file_reader\\data_bela\\4. database_entries\\{id}_bela_entries.csv", id= RUN_OPTS.id)
            }
        }
    }
}

pub const RUN_OPTS: &'static RunOpts = &RunOpts {
    language: Language::Russian,
    id: 1,
    input_wiki_dump: ".xml",
};

fn main() {
    processes::_1_dump_to_intermediary::run().expect("Success of dump to intermediary");
    processes::_2a_intermediary_to_lemmas_a::run().expect("success of 2a intermediary to lemmas_a");
    processes::_2b_intermediary_to_wikipages::run().expect("_2b_intermediary_to_wikipages");
    processes::_3_wikipage_to_database::run().expect("");
}
