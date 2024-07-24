use std::fs::File;
use std::io::Read;

use crate::models::WikiTextPage;
use crate::RUN_OPTS;
use crate::{constants::INTERMEDIARY_DELIMITER, models};

struct Score {
    pos_error_count: u32,
    missing_extra_count: u32,
    inflected_form_page: u32,
    undeclinable_noun_page: u32,
    non_substantive: u64,
    successful: u64,
}
impl Score {
    fn new() -> Self {
        Self {
            pos_error_count: 0,
            missing_extra_count: 0,
            inflected_form_page: 0,
            undeclinable_noun_page: 0,
            non_substantive: 0,
            successful: 0,
        }
    }

    fn readout(self) -> String {
        format!(
            "pos_error_count={pos_error_count}
    non_substantive={non_substantive}
    missing_extra_count={missing_extra_count}
    inflected_form_page={inflected_form_page}
    undeclinable_noun_page={undeclinable_noun_page}
    successful={successful}",
            pos_error_count = self.pos_error_count,
            missing_extra_count = self.missing_extra_count,
            successful = self.successful,
            inflected_form_page = self.inflected_form_page,
            undeclinable_noun_page = self.undeclinable_noun_page,
            non_substantive = self.non_substantive
        )
    }
}

/// Takes in a Wiki dump pages file with equal sign delimiters, and parses into a list of WikiPage objects
pub fn run() -> Result<(), String> {
    let mut intermediary_text = String::with_capacity(50_000);
    File::options()
        .read(true)
        .open(&RUN_OPTS.intermediary_out())
        .unwrap()
        .read_to_string(&mut intermediary_text);

    // Performance trackers
    let mut score = Score::new();
    let mut wiki_pages: Vec<WikiTextPage> = Vec::with_capacity(50_000);

    for page in intermediary_text.split(INTERMEDIARY_DELIMITER) {
        if page == "" { continue; }

        let parse_result = WikiTextPage::parse_from_page(&page, false);
        match parse_result {
            Err(init_error) => match init_error {
                models::WikiTextPageInitError::MissingExtraPiece(msg) => { score.missing_extra_count += 1; }
                models::WikiTextPageInitError::UnimplementedPOSFound(msg) => {score.pos_error_count += 1; }
                models::WikiTextPageInitError::NotADictionaryPage(msg) => (),
                models::WikiTextPageInitError::NotASubstantiveWord(msg) => { score.non_substantive += 1; }
                models::WikiTextPageInitError::InflectedFormPage => { score.inflected_form_page += 1; }
                models::WikiTextPageInitError::UndeclinableNoun => { score.undeclinable_noun_page += 1; }
                models::WikiTextPageInitError::MissingCorePiece(msg) => panic!(
                    "\n
=======================================================start\n
{page}
=======================================================end\n\n
{msg}\n\n
{score}
", score = score.readout() 
                ),
            },
            Ok(wiki_page) => {
                score.successful += 1;
                wiki_pages.push(wiki_page);
            }
        }
    }

    println!("Finished 2b: Intermediary to Wikipage! 🗳️\n{}",score.readout());

    let mut output = File::open(RUN_OPTS.wiki_pages_out()).expect("open output");
    serde_json::to_writer(output, &wiki_pages).expect("writing to json file");

    Ok(())
}
