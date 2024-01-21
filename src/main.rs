pub mod markdown_parser;


use futures::prelude::*;

use clap::Parser;
use markdown_parser::ExtractedTextFromFile;
use orca::{
    llm::{bert::Bert, Embedding, EmbeddingResponse},
    pipeline::simple::LLMPipeline,
    pipeline::Pipeline,
    prompt,
    prompt::context::Context,
    prompts, 
    qdrant::{Qdrant, self},
    record::{Spin, Record}
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};


async fn get_embeddings_and_insert(extracted_texts: Vec<ExtractedTextFromFile>, bert: &Bert, qdrant: &Qdrant, collection: &str) {
    // Insert into Qdrant
    futures::stream::iter(extracted_texts).for_each(|extracted_text| async {
        if qdrant.create_collection(&collection, 384).await.is_ok() {
            let markdown_records = Record::new(orca::record::Content::String(extracted_text.content)).split(399);
            let embeddings = bert.generate_embeddings(prompts!(&markdown_records)).await.unwrap();
            qdrant.insert_many(&collection, embeddings.to_vec2().unwrap(), markdown_records).await.unwrap();
        }
    }).await;
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

async fn get_query_embedding(query_str: String, bert: &Bert) -> EmbeddingResponse {
    let query_embedding = bert.generate_embedding(prompt!(query_str)).await.unwrap();
    return query_embedding;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let obsidian_directory = args.directory;
    let query_prompt = args.prompt;

    let bert = Bert::new().build_model_and_tokenizer().await.unwrap();
    let qdrant = Qdrant::new("http://localhost:6334").unwrap();
    let collection = "default_collection";

    let extracted_text_from_md_files = markdown_parser::extract_text_from_markdown_directory(obsidian_directory.as_str());
    // Get embeddings and insert into qdrant
    get_embeddings_and_insert(extracted_text_from_md_files, &bert, &qdrant, collection).await;

    // Get the query embeddings
    let query_embeddings = get_query_embedding(query_prompt, &bert).await;   
    let result = qdrant.search(collection, query_embeddings.to_vec().unwrap().clone(), 5, None).await.unwrap();
}
