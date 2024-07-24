use crate::{RUN_OPTS, traits::Searchable};
use super::utils::redact_comment_block;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

const PAGE_START_TAG: &[u8; 6] = b"<page>";
const PAGE_END_TAG: &[u8; 7] = b"</page>";


/// Takes a wikidump xml file and produces an intermediary file filters for selected languages
///
/// Isolates, when set to Russian:
/// <page>
/// ...
///
/// ===Russian===
///
///
/// ===Foo===
/// ...
/// </page>
///
/// Pages without selected language excluded (massively reducing file size)
///
/// Importantly, the <comment></comment> blocks are also attempt-removed, since they often contain differently patterned similar information as what we are trying to pinpoint and parse!
/// 
pub fn run() -> Result<(), &'static str> {
    // Read Dump
    let file = File::open(RUN_OPTS.input_wiki_dump).expect("Failed to open file");
    let mut reader = BufReader::new(file);

    // Buffer to hold the data read from the file (Have to initialize like this because otherwise it will overflow stack before being moved over)
    let proto_buf: Vec<u8> = vec![
        0; // Fill
        1024 // kilobyte
     * 1024 // megabyte
     * 500 //
    ];
    let mut temp_buffer: Box<[u8]> = proto_buf.into_boxed_slice();
    // Produce Intermediary File

    let out_file = File::options()
        .create_new(true)
        .write(true)
        .open(&RUN_OPTS.intermediary_out())
        .expect("No existing file may be found at attempted out-write location");
    let mut out_buffer = BufWriter::new(out_file);

    print!("Processing wikidump into intermediary file...");
    std::io::stdout().flush().unwrap();

    loop {
        let bytes_read = reader
            .read(&mut temp_buffer)
            .expect("Failed to read from file");
        if bytes_read == 0 {
            break;
        }

        process_buffer(&mut temp_buffer[..bytes_read], &mut out_buffer);
        out_buffer.flush().expect("Flush");
    }

    print!(" Success!");
    std::io::stdout().flush().unwrap();

    Ok(())
}

///
/// Takes a buffer of bytes, finds the <page> starting tags and relevant language starting header
pub fn process_buffer(buffer: &mut[u8], output_buffer: &mut BufWriter<File>) {
    let page_start_tag_occurrences = buffer.as_ref().find_all(PAGE_START_TAG);

    let language_section_delimiter: &[u8] = match RUN_OPTS.language {
        crate::models::Language::Russian => b"==Russian==",
        crate::models::Language::Ukrainian => b"==Ukrainian==",
        crate::models::Language::Belarusian => b"==Belarusian==",
    };

    for page_start_pos in page_start_tag_occurrences {
        if let Some(page_end_pos) = &buffer[page_start_pos..].as_ref().find(PAGE_END_TAG) {
            let page_end_pos = page_start_pos + page_end_pos + PAGE_END_TAG.len();
            let mut page_content = &mut buffer[page_start_pos..page_end_pos];

            if let Some(_) = page_content.as_ref().find(&language_section_delimiter) {

                redact_comment_block(&mut page_content);

                output_buffer
                    .write(b"===================================================\n")
                    .expect("File write");
                output_buffer.write(page_content).expect("File write");
                output_buffer.write(b"\n").expect("File write");
            }
        }
    }
}
