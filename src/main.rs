use gcmd::Chunker;
use std::{env, fs};
use tokenizers::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let markdown = fs::read_to_string(file_path).expect("Failed to read the markdown file");

    let tokenizer =
        Tokenizer::from_pretrained("facebook/bart-base", None).expect("Failed to load tokenizer");

    let chunker = Chunker::new(1024, &tokenizer).with_sticky_headers(true);

    match chunker.chunk(&markdown) {
        Ok(chunks) => {
            for (i, chunk) in chunks.iter().enumerate() {
                println!("Chunk {}:\n{}\n", i + 1, chunk);
            }
        }
        Err(err) => eprintln!("Error chunking Markdown: {:?}", err),
    }
}
