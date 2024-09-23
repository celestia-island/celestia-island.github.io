use chrono::Datelike;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentNewsDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl From<chrono::DateTime<chrono::Utc>> for RecentNewsDate {
    fn from(date: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentNewsItem {
    pub id: i64,
    pub title: String,
    pub updated: RecentNewsDate,
}

pub type RecentNews = Vec<RecentNewsItem>;
