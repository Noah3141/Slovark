use std::fs::File;
use std::io::Read;

use crate::models::WikiTextPage;
use crate::RUN_OPTS;
use crate::constants::INTERMEDIARY_DELIMITER;
use crate::models::wiki_text_page::error::Score;



/// Takes in a Wiki dump pages file with equal sign delimiters, and parses into a list of WikiPage objects
pub fn run() -> Result<(), String> {
    let mut intermediary_text = String::with_capacity(50_000);
    File::options()
        .read(true)
        .open(&RUN_OPTS.intermediary_out())
        .unwrap()
        .read_to_string(&mut intermediary_text)
        .expect("reading to string");

    // Performance trackers
    let mut score = Score::new();
    let mut wiki_pages: Vec<WikiTextPage> = Vec::with_capacity(50_000);

    for page in intermediary_text.split(INTERMEDIARY_DELIMITER) {
        if page == "" { continue; }

        let parse_result = WikiTextPage::parse_from_page(&page, false);
        match parse_result {
            Err(init_error) =>  init_error.reflect_in(&mut score, page),
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
