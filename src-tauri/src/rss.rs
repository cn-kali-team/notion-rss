use anyhow::{anyhow, Result};
use futures::channel::mpsc::unbounded;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use notion_sdk::common::parent::Parent;
use std::collections::HashMap;

use crate::{ArchivePage, SourcePage, NOTION_FEED};
use notion_sdk::database::properties::{Properties, PropertyValue};
use notion_sdk::pagination::{Object, Pageable, PagingCursor};
use notion_sdk::search::{
    CheckboxCondition, DatabaseQuery, FilterCondition, PropertyCondition, TextCondition,
};

async fn get_source(start_pages: &Option<PagingCursor>) -> Result<Object> {
    // Filter the source `Enabled` by the checkbox
    let query = DatabaseQuery {
        sorts: None,
        filter: Some(FilterCondition::And {
            and: vec![
                FilterCondition::Property {
                    property: "Enabled".to_string(),
                    condition: PropertyCondition::Checkbox(CheckboxCondition::Equals(true)),
                },
                FilterCondition::Property {
                    property: "Skip".to_string(),
                    condition: PropertyCondition::Checkbox(CheckboxCondition::Equals(false)),
                },
            ],
        }),
        paging: None,
    }
    .start_from(start_pages.clone());
    let dbs = NOTION_FEED
        .notion
        .databases_query(NOTION_FEED.source_id.clone(), Some(query))
        .await?;
    Ok(dbs)
}

pub async fn update(window: Option<tauri::Window>) {
    let (mut page_sender, mut page_receiver) = unbounded();
    let o_window = window.clone();
    let source_handle = tokio::task::spawn(async move {
        let mut start_pages: Option<PagingCursor> = None;
        loop {
            match get_source(&start_pages).await {
                Ok(Object::List { list }) => {
                    start_pages = list.next_cursor;
                    for pages in list.results {
                        if let Object::Page { page, .. } = pages {
                            page_sender
                                .start_send(SourcePage::from_page(&page))
                                .unwrap_or_default();
                        }
                    }
                    if !list.has_more {
                        break;
                    }
                }
                Err(err) => {
                    if let Some(w) = o_window.clone() {
                        w.emit("INFO", err.to_string()).unwrap_or_default();
                    } else {
                        println!("Update failed: {}", err);
                    }
                    return format!("Get Source Error: {}", err);
                }
                _ => {
                    break;
                }
            }
        }
        String::new()
    });
    let rss_handle = tokio::task::spawn(async move {
        let mut worker = FuturesUnordered::new();
        for _ in 0..NOTION_FEED.config.thread {
            match page_receiver.next().await {
                Some(sp) => worker.push(sp.get_feed()),
                None => {
                    break;
                }
            }
        }
        while let Some(result) = worker.next().await {
            if let Some(sp) = page_receiver.next().await {
                worker.push(sp.get_feed());
            }
            match result {
                Ok(result) => {
                    if let Some(w) = window.clone() {
                        w.emit("INFO", result.to_string()).unwrap_or_default();
                    } else {
                        println!("Update succeeded: {}", result);
                    }
                }
                Err(err) => {
                    if let Some(w) = window.clone() {
                        w.emit("ERROR", err.to_string()).unwrap_or_default();
                    } else {
                        println!("Update failed: {}", err)
                    }
                }
            }
        }
        String::new()
    });
    let (_r1, _r2) = tokio::join!(source_handle, rss_handle);
}

// Add subscription link from RssHub browser plug-in
pub async fn add_subscribe(u: String) -> Result<String> {
    // SourcePage
    if let Err(e) = reqwest::Url::parse(&u) {
        return Err(anyhow!(format!("Submitted Failed: {}.", e)));
    }
    if let Ok(e) = filter_from_database(u.clone()).await {
        return Err(anyhow!(format!("The feed already exists as :{}", e)));
    }
    let mut page_properties = HashMap::new();
    page_properties.insert(
        "Link".to_string(),
        PropertyValue::Url {
            id: Default::default(),
            url: Some(u.clone()),
        },
    );
    page_properties.insert(
        "Enabled".to_string(),
        PropertyValue::Checkbox {
            id: Default::default(),
            checkbox: true,
        },
    );
    let page = notion_sdk::pages::CreatePage {
        icon: None,
        parent: Parent::Database {
            database_id: NOTION_FEED.source_id.clone(),
        },
        properties: Properties {
            properties: page_properties,
        },
        children: vec![],
    };
    if let Err(e) = NOTION_FEED.notion.pages_create(page).await {
        return Err(anyhow!(e));
    }

    Ok(u.clone())
}

// Check if the feed already exists
async fn filter_from_database(url: String) -> Result<String> {
    let query = DatabaseQuery {
        sorts: None,
        filter: Some(FilterCondition::Property {
            property: "Link".to_string(),
            condition: PropertyCondition::RichText(TextCondition::Equals(url)),
        }),
        paging: None,
    };
    let dbs = NOTION_FEED
        .notion
        .databases_query(NOTION_FEED.source_id.clone(), Some(query))
        .await?;
    if let Object::List { list } = dbs {
        for page in list.results {
            if let Object::Page { page } = page {
                let tit = page.get_title();
                return Ok(tit);
            }
        }
    }
    Err(anyhow!("NotFound"))
}

async fn get_deleted_page(start_pages: &Option<PagingCursor>) -> Result<Object> {
    // Filter the source `Enabled` by the check box
    let query = DatabaseQuery {
        sorts: None,
        filter: Some(FilterCondition::And {
            and: vec![FilterCondition::Property {
                property: "Deleted".to_string(),
                condition: PropertyCondition::Checkbox(CheckboxCondition::Equals(true)),
            }],
        }),
        paging: None,
    }
    .start_from(start_pages.clone());
    let dbs = NOTION_FEED
        .notion
        .databases_query(NOTION_FEED.archive_id.clone(), Some(query))
        .await?;
    Ok(dbs)
}

pub async fn deleted() {
    let (mut page_sender, mut page_receiver) = unbounded();
    let archive_handle = tokio::task::spawn(async move {
        let mut start_pages: Option<PagingCursor> = None;
        loop {
            match get_deleted_page(&start_pages).await {
                Ok(Object::List { list }) => {
                    start_pages = list.next_cursor;
                    for pages in list.results {
                        if let Object::Page { page, .. } = pages {
                            page_sender
                                .start_send(ArchivePage::from_page(&page))
                                .unwrap_or_default();
                        }
                    }
                    if !list.has_more {
                        break;
                    }
                }
                Err(e) => {
                    println!("Get Deleted Page Error: {}", e);
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        true
    });
    let deleted_handle = tokio::task::spawn(async move {
        let mut worker = FuturesUnordered::new();
        for _ in 0..NOTION_FEED.config.thread {
            match page_receiver.next().await {
                Some(ap) => worker.push(ap.delete_me()),
                None => {
                    break;
                }
            }
        }
        while (worker.next().await).is_some() {
            if let Some(ap) = page_receiver.next().await {
                worker.push(ap.delete_me());
            }
        }
        true
    });
    let (_r1, _r2) = tokio::join!(archive_handle, deleted_handle);
}
