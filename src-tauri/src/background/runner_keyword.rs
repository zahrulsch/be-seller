use entity::config::Model;
use std::sync::Arc;
use std::iter;

fn chunker(into_chunk: Vec<String>, size: usize) -> Vec<Vec<String>> {
    let mut chunks = iter::repeat(
        Vec::<String>::new()).take(size
    ).collect::<Vec<_>>();
    let mut into_chunks = into_chunk.into_iter();

    'outer: loop {
        for (i, _) in chunks.clone().iter().enumerate() {
            let Some(uri) = into_chunks.next() else {
                break 'outer; 
            };

            chunks[i].push(uri);
        }
    }

    chunks
}

#[derive(Debug)]
pub struct RunnerKeywordData {
    pub model: Model,
    pub keywords: Vec<String>,
    pub thread_size: u8,
    pub name: String,
    pub limit_product: u16,
}

pub async fn runner_keyword(data: RunnerKeywordData) {
    let _config_arc = Arc::new(data.model);
    let chunks = self::chunker(data.keywords, data.thread_size.into());
    println!("{:#?}", chunks);
}