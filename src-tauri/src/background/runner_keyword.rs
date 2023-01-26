use entity::config::Model;
use std::sync::{Arc, Mutex};
use std::iter;
use tokio::{sync::mpsc, task::JoinHandle};
use auto_requester::{Shopee, CommonRequester, UrlType, ReqMethod};

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
    let mut processes = Vec::<JoinHandle<()>>::new();
    
    // Mulai dari sini akan diloop sync per halaman
    // 
    let products = Vec::<Vec<String>>::new();
    let products = Arc::new(Mutex::new(products));

    for chunk in chunks {
        let products = products.clone();
        let keywords = chunk.clone();
        let (tx, mut rc) = mpsc::channel(100);

        let sender = tokio::spawn(async move {
            for keyword in keywords {
                if tx.send(keyword).await.is_err() {
                    eprintln!("failed send msg")
                }
            }
        });

        let receiver = tokio::spawn(async move {
            while let Some(keyword) = rc.recv().await {
                let url = format!(
                    "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword={}&newest=20&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2",
                    keyword
                );
                let mut client = Shopee::new(url, UrlType::Search, ReqMethod::Get);
                client.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");

                match client.get_search_data().await {
                    Ok(resp) => {
                        let items = resp.items.iter().map(|i| i.item_basic.name.clone()).collect::<Vec<_>>();
                        products.lock().unwrap().push(items)
                    },
                    Err(e) => {
                        eprintln!("gagal dapat data -> {}, error: {:#?}", keyword, e)
                    }
                }
            }
        });

        processes.push(receiver);
        processes.push(sender);
    }

    for process in processes {
        process.await.unwrap()
    }

    println!("{:?}", products.lock().unwrap())
}