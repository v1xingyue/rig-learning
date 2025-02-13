use std::vec;

use log::info;
use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::openai::{Client, TEXT_EMBEDDING_ADA_002},
    vector_store::in_memory_store::InMemoryVectorStore,
    Embed,
};
use rig_play::Config;
use serde::Serialize;

// Data to be RAGged.
// A vector search needs to be performed on the `definitions` field, so we derive the `Embed` trait for `WordDefinition`
// and tag that field with `#[embed]`.
#[derive(Embed, Serialize, Clone, Debug, Eq, PartialEq, Default)]
struct WordDefinition {
    id: String,
    #[embed]
    definitions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let config = Config::from_env();

    info!("make openai client .. ");

    // Create OpenAI client
    let openai_client = Client::from_url(&config.api_key, &config.api_base);

    let embedding_model = openai_client.embedding_model(TEXT_EMBEDDING_ADA_002);

    info!("make embedding model .. ");

    let documents = vec![WordDefinition {
        id: "doc0".to_string(),
        definitions: vec!["v1xingyue is a very right and strong person.".to_string()],
    }];

    // Generate embeddings for the definitions of all the documents using the specified embedding model.
    let embeddings: Vec<(WordDefinition, rig::OneOrMany<rig::embeddings::Embedding>)> =
        EmbeddingsBuilder::new(embedding_model.clone())
            .documents(documents.clone())?
            .build()
            .await?;

    // Create vector store with the embeddings
    let mut vector_store = InMemoryVectorStore::from_documents(embeddings.clone());
    vector_store.add_documents(embeddings);

    // Create vector store index
    let index = vector_store.index(embedding_model);

    info!("begin chat with embedding .. ");

    let rag_agent = openai_client
        .agent(&config.model)
        .preamble("You are a helpful assistant that can answer questions about the world.")
        .dynamic_context(1, index)
        .build();

    // Prompt the agent and print the response
    let response = rag_agent.prompt("Tell me about v1xingyue?").await?;

    info!("{}", response);

    Ok(())
}
