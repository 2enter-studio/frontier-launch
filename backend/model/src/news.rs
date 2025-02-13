use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::error::Error;
use thirtyfour::prelude::*;
use utils::runtime::rand_sleep;
use uuid::Uuid;

const URL: &str =
    "https://news.google.com/search?q=%E5%A4%AA%E7%A9%BA&hl=zh-TW&gl=TW&ceid=TW%3Azh-Hant";

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct News {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    hype: i32,
    title: String,
}

impl News {
    pub async fn get_10(pool: &PgPool) -> Vec<String> {
        let mut news: Vec<Self> =
            sqlx::query_as("SELECT * from news ORDER BY updated_at DESC LIMIT 30")
                .fetch_all(pool)
                .await
                .unwrap_or_default();

        // sort news by hype
        news.sort_by(|a, b| b.hype.cmp(&a.hype));
        news.truncate(10);
        news.iter().map(|n| n.title.clone()).collect()
    }

    pub async fn fetch_remote(pool: &PgPool, driver: &WebDriver) -> Result<(), Box<dyn Error>> {
        println!("fetching news...");
        driver.goto(URL).await?;
        // scroll down
        rand_sleep(3000).await;
        driver
            .execute(
                "window.scrollTo(0, document.body.scrollHeight);",
                Vec::new(),
            )
            .await?;
        rand_sleep(3000).await;

        // get sections
        let sections = driver.find_all(By::Css("c-wiz .PO9Zff")).await?;
        println!("{sections:?}");

        let mut titles = Vec::new();

        rand_sleep(3000).await;
        // get titles in sections
        for section in sections {
            let title = section.find(By::Css(".JtKRv")).await?.text().await?;
            println!("fetched news: {title}");
            titles.push(title);
        }

        rand_sleep(3000).await;
        // update existed news
        let existed_titles: Vec<String> = sqlx::query!(
            "UPDATE news SET hype = hype + 1, updated_at = now() WHERE title IN (SELECT unnest($1::VARCHAR[])) RETURNING *",
            &titles
        )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|news| news.title).collect();

        // filter out existed news
        let new_titles: Vec<String> = titles
            .into_iter()
            .filter(|title| !existed_titles.contains(title))
            .collect();

        // insert new news
        for title in new_titles {
            let result = sqlx::query!(
                "INSERT INTO news (title) VALUES ($1) RETURNING title",
                title
            )
            .fetch_one(pool)
            .await?
            .title;
            println!("inserted news: {result}");
        }

        Ok(())
    }
}
