use entity::{config::Model, collection, collection_product};
use std::{sync::Arc, collections::HashMap};
use std::iter;
use tokio::{sync::mpsc, task::JoinHandle};
use auto_requester::{Shopee, CommonRequester, UrlType, ReqMethod, model_translations::Item};
use async_mutex::Mutex;
use serde_json::Value;
use urlencoding::encode;
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection};

#[derive(Debug, Clone)]
struct Uri {
    url: String,
    params: HashMap<String, String>
}

impl Uri {
    pub fn new<T: Into<String>>(url: T) -> Self {
        let mut params = HashMap::new();

        params.insert("order".to_string(), "desc".to_string());
        params.insert("page_type".to_string(), "search".to_string());
        params.insert("scenario".to_string(), "PAGE_GLOBAL_SEARCH".to_string());
        params.insert("version".to_string(), "2".to_string());

        Self {
            url: url.into(),
            params
        }
    }

    pub fn extend_params<T: Into<String>>(&mut self, key: T, value: T) {
        self.params.insert(key.into(), value.into());
    }

    pub fn parse(&self) -> String {
        let params = self.params.iter().map(|k| format!("{}={}", k.0, k.1)).collect::<Vec<_>>();
        let params = params.join("&");
        let full_url = format!("{}?{}", self.url, params);
        full_url
    }
}

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

fn filter_applier(items: Vec<Vec<Item>>, config_filter: Model) -> Vec<Item> {
    let mut temporary_products = vec![];
    if let Some(longest) = items.iter().map(|i| i.len()).max() {
        for index in 0..=longest {
            for item in &items {
                if let Some(product) = item.get(index) {
                    let mut threshold = vec![] as Vec<bool>;
                    let cities = config_filter.cities.clone();

                    // cek lokasi
                    if let Value::Array(cities) = cities {
                        if cities.iter()
                            .map(|v| {
                               if let Value::String(v) = v {
                                    if product.item_basic.shop_location.to_lowercase().contains(&v.to_lowercase()) {
                                        return true
                                    }
                                    return false
                               }

                               false
                            }).any(|x| x) {
                            threshold.push(true)
                        } else {
                            threshold.push(false)
                        }
                    }

                    // cek title
                    let ban_keywords = config_filter.ban_keywords.clone();
                    if let Value::Array(ban_keywords) = ban_keywords {
                        if !ban_keywords.is_empty() {
                            if ban_keywords.iter()
                                .map(|v| {
                                   if let Value::String(v) = v {
                                        if !product.item_basic.name.to_lowercase().contains(&v.to_lowercase()) {
                                            return false
                                        }
                                        return true
                                   }
    
                                   false
                                }).any(|x| x) {
                                threshold.push(false)
                            } else {
                                threshold.push(true)
                            }
                        }
                    }
                    
                    // let exclude_cod = config_filter.exclude_cod;
                    let minimum_sold = config_filter.minimum_sold;
                    if product.item_basic.historical_sold < minimum_sold.into() {
                        threshold.push(false)
                    } else {
                        threshold.push(true)
                    }
                    
                    let minimum_stock = config_filter.minimum_stock;
                    if product.item_basic.stock < minimum_stock.into() {
                        threshold.push(false)
                    } else {
                        threshold.push(true)
                    }

                    if threshold.iter().all(|cond| cond == &true) {
                        temporary_products.push(product.clone())
                    }
                }
            }
        }
    }   
    temporary_products
}

fn item_to_model(items: Vec<Item>, collection: &collection::Model) -> Vec<collection_product::ActiveModel> {
    let col_id = collection.id;

    items.iter()
        .map(|item| {
            let item_url = format!(
                "https://shopee.co.id/{}-i.{}.{}",
                encode(&item.item_basic.name),
                item.item_basic.shopid,
                item.item_basic.itemid
            );

            let item_model = collection_product::ActiveModel {
                collection_id: Set(col_id),
                item_id: Set(item.item_basic.itemid),
                shop_id: Set(item.item_basic.shopid),
                name: Set(item.item_basic.name.clone()),
                sold: Set(Some(item.item_basic.historical_sold)),
                view: Set(Some(0)),
                price_max: Set(Some(item.item_basic.price_max/100_000)),
                price_min: Set(Some(item.item_basic.price_min/100_000)),
                url: Set(item_url),
                thumbnail: Set(Some(format!("https://cf.shopee.co.id/file/{}", item.item_basic.image)))
            };
            item_model
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
pub struct RunnerKeywordData {
    pub config: Model,
    pub keywords: Vec<String>,
    pub thread_size: u8,
    pub name: String,
    pub limit_product: u16,
    pub collection: collection::Model,
}

#[derive(Debug, Clone)]
pub struct ChannelData {
    pub keyword: String,
    pub session: String,
    pub page: i32,
    pub newest: usize,
}

pub async fn runner_keyword(data: RunnerKeywordData, connection: &DatabaseConnection) {
    let chunks = self::chunker(data.keywords.clone(), data.thread_size.into());
    let channel_data = Arc::new(Mutex::new(Vec::<ChannelData>::new()));
    let product_inputed = Arc::new(Mutex::new(0));
    let limit = data.limit_product;
    
    // Mulai dari sini akan diloop sync per halaman
    'outer: for page in 1..=2000 {
        let mut processes = Vec::<JoinHandle<()>>::new();
        let products = Arc::new(Mutex::new(Vec::new()));
        
        for chunk in &chunks {
            let data = data.config.clone();
            let sort_type = data.sort_by.clone();
            let seller_type = data.seller_types.clone();
            let price_range = data.price_ranges.clone();
            let shipppings = data.shippings.clone();
            let rating = data.minimum_star;

            let keywords = chunk.clone();
            let (tx, mut rc) = mpsc::channel(100);
            let channel_data_tx = channel_data.clone();
            let products = products.clone();
            
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
                    let mut url = Uri::new("https://mall.shopee.co.id/api/v4/search/search_items");

                    url.extend_params("by", &sort_type);
                    url.extend_params("keyword", &encode(&channel_data_parent.keyword));
                    url.extend_params("newest", &encode(&channel_data_parent.newest.to_string()));
                    url.extend_params("page", &encode(&page.to_string()));
                    url.extend_params("rating_filter", &encode(&rating.to_string()));

                    if let Value::Array(ref seller_type) = seller_type {
                        let seller_filter = seller_type.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("%2C");
                        url.extend_params("filter", &seller_filter);
                    }
                    
                    if let Value::Array(ref price_range) = shipppings {
                        let shippings_filter = price_range.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("%2C");
                        url.extend_params("shippings", &shippings_filter);
                    }

                    if let Value::Array(ref price_range) = price_range {
                        if let (Some(min), Some(max)) = (price_range.get(0), price_range.get(1)) {
                            url.extend_params("price_min", &format!("{}", min));
                            url.extend_params("price_max", &format!("{}", max));
                        }
                    }
                    
                    if !channel_data_parent.session.is_empty() {
                        url.extend_params("search_sessionid", &encode(&channel_data_parent.session));
                    }

                    let mut client = Shopee::new(url.parse(), UrlType::Search, ReqMethod::Get);
                    client.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");
    
                    let mut products = products.lock().await;

                    match client.get_search_data().await {
                        Ok(resp) => {
                            if !resp.items.is_empty() {
                                products.push(resp.items.clone());
                            }

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

        let items = filter_applier(products.lock().await.to_vec(), data.clone().config);
        let item_models = item_to_model(items, &data.collection);

        for item_model in item_models {
            let mut counter = product_inputed.lock().await;

            if *counter < limit {
                match item_model.insert(connection).await {
                    Ok(i) => {
                        println!("{} - ditambahkan dengan id: {}", i.name, i.item_id);
                        *counter += 1;
                    },
                    Err(e) => {
                        eprintln!("Gagal menambahkan produk: {:?}", e);
                    }
                }
            } else {
                println!("Limit sudah tercapai...");
                break 'outer;
            }

        }

        if products.lock().await.is_empty() {
            break 'outer;
        }
    }
}