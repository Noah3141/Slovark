use crate::{traits::Searchable, RUN_OPTS};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

/// Takes a wikidump xml file and produces an intermediary file filters for selected languages
///
///
pub fn run() -> Result<(), &'static str> {
    // Read Dump
    let file = File::open(RUN_OPTS.input_wiki_dump).expect("Failed to open file");
    let mut reader = BufReader::new(file);

    // Buffer to hold the data read from the file
    let proto_buf: Vec<u8> = vec![
        0; // Fill
        1024 // kilobyte
     * 1024 // megabyte
     * 180 // 180 MB
    ];
    let mut temp_buffer: Box<[u8]> = proto_buf.into_boxed_slice();
    // Produce Intermediary File

    let out_file = File::options()
        .create(true)
        .write(true)
        .open(&RUN_OPTS.intermediary_out())
        .unwrap();
    let mut out_buffer = BufWriter::new(out_file);

    println!("Starting...");
    loop {
        let bytes_read = reader
            .read(&mut temp_buffer)
            .expect("Failed to read from file");
        if bytes_read == 0 {
            break;
        }

        process_buffer(&temp_buffer[..bytes_read], &mut out_buffer);
        out_buffer.flush().expect("Flush");
    }

    Ok(())
}

const PAGE_START_TAG: &[u8; 6] = b"<page>";
const PAGE_END_TAG: &[u8; 7] = b"</page>";

pub fn process_buffer(buffer: &[u8], output_buffer: &mut BufWriter<File>) {
    let occurrences = buffer.find_all(PAGE_START_TAG);

    let language_section_delimiter: &[u8] = match RUN_OPTS.language {
        crate::models::Language::Russian => b"==Russian==",
        crate::models::Language::Ukrainian => b"==Ukrainian==",
        crate::models::Language::Belarusian => b"==Belarusian==",
    };
    println!("- {}", occurrences.len());

    for start_pos in occurrences {
        if let Some(end_pos) = &buffer[start_pos..].find(PAGE_END_TAG) {
            let end_pos = start_pos + end_pos + PAGE_END_TAG.len();
            let page_content = &buffer[start_pos..end_pos];

            if let Some(_) = page_content.find(&language_section_delimiter) {
                output_buffer
                    .write(b"===================================================\n")
                    .expect("File write");
                output_buffer.write(page_content).expect("File write");
                output_buffer.write(b"\n").expect("File write");
            }
        }
    }
}

pub fn extract_title(page_content: &[u8]) -> Option<&[u8]> {
    let title_start_tag = b"<title>";
    let title_end_tag = b"</title>";

    if let Some(start_pos) = page_content.find(title_start_tag) {
        if let Some(end_pos) = page_content[start_pos..].find(title_end_tag) {
            let title_start = start_pos + title_start_tag.len();
            let title_end = start_pos + end_pos;
            let title_bytes = &page_content[title_start..title_end];

            return Some(title_bytes);
        }
    }
    None
}
