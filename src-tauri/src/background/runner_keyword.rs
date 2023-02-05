use entity::config::Model;
use std::sync::Arc;
use std::iter;
use tokio::{sync::mpsc, task::JoinHandle};
use auto_requester::{Shopee, CommonRequester, UrlType, ReqMethod};
use async_mutex::Mutex;
use serde_json::Value;
use urlencoding::encode;

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

#[derive(Debug, Clone)]
pub struct ChannelData {
    pub keyword: String,
    pub session: String,
    pub page: i32,
    pub newest: usize,
}

pub async fn runner_keyword(data: RunnerKeywordData) {
    let _config_arc = Arc::new(data.model);
    let chunks = self::chunker(data.keywords, data.thread_size.into());
    
    let channel_data = Arc::new(Mutex::new(Vec::<ChannelData>::new()));
    
    // Mulai dari sini akan diloop sync per halaman
    for page in 1..=10 {
        let mut processes = Vec::<JoinHandle<()>>::new();
        let products = Vec::<Vec<String>>::new();
        let products = Arc::new(Mutex::new(products));
    
        for chunk in &chunks {
            let products = products.clone();
            let keywords = chunk.clone();
            let (tx, mut rc) = mpsc::channel(100);
            let channel_data_tx = channel_data.clone();
            
            let sender = tokio::spawn(async move {
                for keyword in keywords {
                    let mut channel_data = channel_data_tx.lock().await;
                    let keyword_channel_data = channel_data
                        .iter()
                        .filter(|cd| cd.keyword == keyword)
                        .collect::<Vec<_>>();

                    let keyword_channel_data = keyword_channel_data.first().cloned();

                    match keyword_channel_data {
                        Some(channel_data) => {
                            if tx.send(channel_data.clone()).await.is_err() {
                                eprintln!("failed send msg")
                            }
                        },
                        None => {
                            let cd = ChannelData {
                                newest: 0,
                                session: "".into(),
                                page,
                                keyword,
                            };

                            channel_data.push(cd.clone());

                            if tx.send(cd).await.is_err() {
                                eprintln!("failed send msg")
                            }
                        }
                    }

                }
            });
    
            let channel_data_rx = channel_data.clone();

            let receiver = tokio::spawn(async move {
                while let Some(channel_data_parent) = rc.recv().await {
                    println!("{:?}", &channel_data_parent);
                    let url = format!(
                        "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword={}&newest={}&order=desc&page={}&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2&search_sessionid={}",
                        encode(&channel_data_parent.keyword),
                        channel_data_parent.newest,
                        page,
                        encode(&channel_data_parent.session)
                    );
                    println!("{}", &url);

                    let mut client = Shopee::new(url, UrlType::Search, ReqMethod::Get);
                    client.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");
    
                    match client.get_search_data().await {
                        Ok(resp) => {
                            println!("{}", resp.items.len());
                            let session = resp.search_sessionid;

                            if let Value::String(s) = session {
                                let mut channel_data = channel_data_rx.lock().await;
                                let channel_data_2 = channel_data.clone();
                                *channel_data = channel_data_2.iter().map(|cd| {
                                    let mut cloning = cd.clone();
    
                                    if cloning.keyword == channel_data_parent.keyword {
                                        cloning.session = s.clone();
                                        cloning.newest += resp.items.len();
                                    }
    
                                    cloning
                                }).collect::<Vec<_>>();
                            }
                        },
                        Err(e) => {
                            eprintln!("gagal dapat data -> {}, error: {:#?}", channel_data_parent.keyword, e)
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
    }
}