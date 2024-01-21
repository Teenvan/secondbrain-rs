pub mod markdown_parser;
use clap::Parser;
use markdown_parser::ExtractedTextFromFile;
use orca::{
    llm::{bert::Bert, Embedding, EmbeddingResponse},
    pipeline::simple::LLMPipeline,
    pipeline::Pipeline,
    prompt,
    prompt::context::Context,
    prompts, 
    qdrant::Qdrant,
    record::{Spin, Record}
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;

fn main() {
    println!("Hello, world!");
}


async fn get_embeddings(extracted_text: ExtractedTextFromFile, bert: &Bert) -> EmbeddingResponse {
    let markdown_records = Record::new(orca::record::Content::String(extracted_text.content)).split(399);
    let embeddings = bert.generate_embeddings(prompts!(&markdown_records)).await.unwrap();
    return embeddings;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    /// The path to the directory containing the MD files to index
    directory: String,

    #[clap(long)]   
    /// The prompt to use to query the index
    prompt: String,
}